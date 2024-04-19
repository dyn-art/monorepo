import type { AdditionalOperation, RulesLogic } from 'json-logic-js';

import type { COMP } from '../comp';

type TMakeEditable<T, K extends keyof T, GKey extends string> = {
	[P in keyof T]: P extends K ? T[P] | GKey : T[P];
};

type TMapInputType<T> = T extends 'NUMBER'
	? number
	: T extends 'STRING'
		? string
		: T extends 'BOOLEAN'
			? boolean
			: never;

type EditableDtifInputEvent<GKey extends string> =
	| ({ type: 'EditableEntityMoved' } & TMakeEditable<COMP.DtifEntityMovedEvent, 'dx' | 'dy', GKey>)
	| ({ type: 'EditableEntitySetPosition' } & TMakeEditable<
			COMP.DtifEntitySetPositionEvent,
			'x' | 'y',
			GKey
	  >);

interface TTemplate<GKey extends string, GInputType extends TInputType> {
	trigger: TTrigger<GKey, GInputType>;
	actions: TAction<GKey>[];
}

type TInputType = 'NUMBER' | 'STRING' | 'BOOLEAN';

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

function createTemplate<
	GKey extends string,
	GInferredKey extends GKey,
	GInputType extends TInputType
>(
	trigger: TTrigger<GKey, GInputType>,
	actions: TAction<GInferredKey>[]
): TTemplate<GKey, GInputType> {
	return { trigger, actions };
}

type TTemplateValue<GKey extends string, TType extends TInputType> = {
	[key in GKey]: TMapInputType<TType>;
};

function processTemplate<GKey extends string, GInputType extends TInputType>(
	template: TTemplate<GKey, GInputType>,
	value: TTemplateValue<GKey, GInputType>
): void {
	// TODO
}

const template = createTemplate(
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

processTemplate(template, { moveX: 10 });
