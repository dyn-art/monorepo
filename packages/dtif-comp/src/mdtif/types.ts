import type { AdditionalOperation, RulesLogic } from 'json-logic-js';

import type { COMP } from '../comp';

export interface TMdtifComposition {
	template: COMP.DtifComposition;
	modificationFields: TModificationField[];
}

export interface TModificationField<
	GKey extends string = string,
	GInputType extends TModificationInputType = TModificationInputType,
	GInferredKey extends GKey = GKey
> {
	key: GKey;
	inputType: GInputType;
	displayName: string;
	actions: TModificationAction<GInferredKey, TMapToDefaultType<GInputType>>[];
}

export interface TModificationAction<GKey extends string, GValue> {
	conditions: TModificationCondition[];
	// compute?: TJsonFunction;
	events: TMdtifInputEvent<GKey, GValue>[];
}

export interface TModificationCondition {
	condition: RulesLogic<AdditionalOperation>;
	notMetMessage?: string;
}

type TMakeEventModifiable<T, K extends keyof T, GKey extends string, GValue> = {
	[P in keyof T]: P extends K ? T[P] | { var: TExpandKey<GKey, GValue> } : T[P];
};

export type TMdtifInputEvent<GKey extends string, GValue> =
	| ({ type: 'EntityMoved' } & TMakeEventModifiable<
			COMP.DtifEntityMovedEvent,
			'dx' | 'dy',
			GKey,
			GValue
	  >)
	| ({ type: 'EntitySetPosition' } & TMakeEventModifiable<
			COMP.DtifEntitySetPositionEvent,
			'x' | 'y',
			GKey,
			GValue
	  >);

export type TModificationInputType =
	| TNumberModificationInput
	| TStringModificationInput
	| TBooleanModificationInput
	| TRangeModificationInput
	| TColorModificationInput
	| TPositionModificationInput;

export interface TNumberModificationInput {
	type: 'NUMBER';
	default: number;
	max?: number;
	min?: number;
}

export interface TStringModificationInput {
	type: 'STRING';
	default: string;
}

export interface TBooleanModificationInput {
	type: 'BOOLEAN';
	default: boolean;
}

export interface TRangeModificationInput {
	type: 'RANGE';
	default: number;
	start: number;
	stop: number;
}

export interface TColorModificationInput {
	type: 'COLOR';
	default: { r: number; g: number; b: number };
}

export interface TPositionModificationInput {
	type: 'POSITION';
	default: [number, number];
	max?: [number, number];
	min?: [number, number];
}

export type TMapToDefaultType<T> = T extends { default: infer U } ? U : never;

type TExpandKey<GKey extends string, GValue> = GValue extends any[]
	? `${GKey}.${number}`
	: GValue extends object
		? { [P in keyof GValue]: `${GKey}.${P & string}` }[keyof GValue]
		: GKey;

export type TFieldData<GKey extends string, GInputType extends TModificationInputType> = {
	[key in GKey]: TMapToDefaultType<GInputType>;
};
