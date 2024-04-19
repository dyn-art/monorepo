import type { AdditionalOperation, RulesLogic } from 'json-logic-js';

import type { COMP } from '../comp';

export interface TEditableDtifComposition {
	template: string | COMP.DtifComposition;
	fields: TField[];
}

export interface TField<
	GKey extends string = string,
	GInputType extends TInputType = TInputType,
	GInferredKey extends GKey = GKey
> {
	key: GKey;
	inputType: GInputType;
	displayName: string;
	actions: TAction<GInferredKey>[];
}

export interface TAction<GKey extends string> {
	conditions: TCondition[];
	// compute?: TJsonFunction;
	events: (COMP.DtifInputEvent | EditableDtifInputEvent<GKey>)[];
}

export interface TCondition {
	condition: RulesLogic<AdditionalOperation>;
	notMetMessage?: string;
}

type TMakeEditable<T, K extends keyof T, GKey extends string> = {
	[P in keyof T]: P extends K ? T[P] | { var: GKey } : T[P];
};

export type EditableDtifInputEvent<GKey extends string> =
	| ({ type: 'EditableEntityMoved' } & TMakeEditable<COMP.DtifEntityMovedEvent, 'dx' | 'dy', GKey>)
	| ({ type: 'EditableEntitySetPosition' } & TMakeEditable<
			COMP.DtifEntitySetPositionEvent,
			'x' | 'y',
			GKey
	  >);

export type TInputType =
	| { type: 'NUMBER'; default: number }
	| { type: 'STRING'; default: string }
	| { type: 'BOOLEAN'; default: boolean }
	| { type: 'RANGE'; default: number; start: number; stop: number }
	| { type: 'COLOR'; default: COMP.Color };

export type TMapInputType<T> = T extends { type: 'NUMBER' }
	? number
	: T extends { type: 'RANGE' }
		? number
		: T extends { type: 'STRING' }
			? string
			: T extends { type: 'BOOLEAN' }
				? boolean
				: T extends { type: 'COLOR' }
					? boolean
					: never;

export type TFieldData<GKey extends string, TType extends TInputType> = {
	[key in GKey]: TMapInputType<TType>;
};
