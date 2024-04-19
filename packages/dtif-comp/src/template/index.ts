import type { AdditionalOperation, RulesLogic } from 'json-logic-js';

import type { COMP } from '../comp';

type TMakeEditable<T, K extends keyof T, GKey extends string> = {
	[P in keyof T]: P extends K ? T[P] | GKey : T[P];
};

type EditableDtifInputEvent<GKey extends string> =
	| ({ type: 'EditableEntityMoved' } & TMakeEditable<COMP.DtifEntityMovedEvent, 'dx' | 'dy', GKey>)
	| ({ type: 'EditableEntitySetPosition' } & TMakeEditable<
			COMP.DtifEntitySetPositionEvent,
			'x' | 'y',
			GKey
	  >);

type TInputType = 'NUMBER' | 'STRING' | 'BOOLEAN';

type TMapInputType<T> = T extends 'NUMBER'
	? number
	: T extends 'STRING'
		? string
		: T extends 'BOOLEAN'
			? boolean
			: never;

interface TTemplate {
	fields: TField;
}

interface TField<GKey extends string = string, GInputType extends TInputType = TInputType> {
	trigger: TTrigger<GKey, GInputType>;
	actions: TAction<GKey>[];
}

interface TTrigger<GKey extends string, GInputType extends TInputType> {
	key: GKey;
	inputType: GInputType;
	displayName: string;
}

interface TAction<GKey extends string> {
	conditions: TCondition[];
	events: (COMP.DtifInputEvent | EditableDtifInputEvent<GKey>)[];
}

interface TCondition {
	condition: RulesLogic<AdditionalOperation>;
	errorMessage?: string;
}

type TTemplateValue<GKey extends string, TType extends TInputType> = {
	[key in GKey]: TMapInputType<TType>;
};

function createField<GKey extends string, GInferredKey extends GKey, GInputType extends TInputType>(
	trigger: TTrigger<GKey, GInputType>,
	actions: TAction<GInferredKey>[]
): TField<GKey, GInputType> {
	return { trigger, actions };
}

function processField<GKey extends string, GInputType extends TInputType>(
	template: TField<GKey, GInputType>,
	value: TTemplateValue<GKey, GInputType>
): void {
	// TODO
}

const field = createField(
	{
		key: 'moveX',
		displayName: 'Move X',
		inputType: 'NUMBER'
	},
	[
		{
			conditions: [
				{ condition: { '>': ['moveX', 0] }, errorMessage: "'moveX' can not be negative!" }
			],
			events: [{ type: 'EditableEntityMoved', entity: 'n1', dx: 'moveX', dy: 0 }]
		}
	]
);

processField(field, { moveX: 10 });
