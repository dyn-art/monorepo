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
	actions: TAction<GInferredKey, TMapInputType<GInputType>>[];
}

export interface TAction<GKey extends string, GValue> {
	conditions: TCondition[];
	// compute?: TJsonFunction;
	events: (COMP.DtifInputEvent | EditableDtifInputEvent<GKey, GValue>)[];
}

export interface TCondition {
	condition: RulesLogic<AdditionalOperation>;
	notMetMessage?: string;
}

type TMakeEditable<T, K extends keyof T, GKey extends string, GValue> = {
	[P in keyof T]: P extends K ? T[P] | { var: TExpandKey<GKey, GValue> } : T[P];
};

export type EditableDtifInputEvent<GKey extends string, GValue> =
	| ({ type: 'EditableEntityMoved' } & TMakeEditable<
			COMP.DtifEntityMovedEvent,
			'dx' | 'dy',
			GKey,
			GValue
	  >)
	| ({ type: 'EditableEntitySetPosition' } & TMakeEditable<
			COMP.DtifEntitySetPositionEvent,
			'x' | 'y',
			GKey,
			GValue
	  >);

type TExpandKey<GKey extends string, GValue> = GValue extends any[]
	? `${GKey}.${number}`
	: GValue extends object
		? { [P in keyof GValue]: `${GKey}.${P & string}` }[keyof GValue]
		: GKey;

export type TInputType =
	| { type: 'NUMBER'; default: number }
	| { type: 'STRING'; default: string }
	| { type: 'BOOLEAN'; default: boolean }
	| { type: 'RANGE'; default: number; start: number; stop: number }
	| { type: 'COLOR'; default: { r: number; g: number; b: number } }
	| { type: 'POSITION'; default: [number, number] };

export type TMapInputType<T> = T extends { default: infer U } ? U : never;

export type TFieldData<GKey extends string, GInputType extends TInputType> = {
	[key in GKey]: TMapInputType<GInputType>;
};
