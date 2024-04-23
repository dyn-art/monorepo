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
	events: (COMP.DtifInputEvent | TMdtifInputEvent<GKey, GValue>)[];
}

export interface TModificationCondition {
	condition: RulesLogic<AdditionalOperation>;
	notMetMessage?: string;
}

type TMakeEventModifiable<T, K extends keyof T, GKey extends string, GValue> = {
	[P in keyof T]: P extends K ? T[P] | { var: TExpandKey<GKey, GValue> } : T[P];
};

export type TMdtifInputEvent<GKey extends string, GValue> =
	| ({ type: 'EditableEntityMoved' } & TMakeEventModifiable<
			COMP.DtifEntityMovedEvent,
			'dx' | 'dy',
			GKey,
			GValue
	  >)
	| ({ type: 'EditableEntitySetPosition' } & TMakeEventModifiable<
			COMP.DtifEntitySetPositionEvent,
			'x' | 'y',
			GKey,
			GValue
	  >);

export type TMdtifInputEventType = `Editable${COMP.DtifInputEvent['type']}`;

export type TModificationInputType =
	| { type: 'NUMBER'; default: number }
	| { type: 'STRING'; default: string }
	| { type: 'BOOLEAN'; default: boolean }
	| { type: 'RANGE'; default: number; start: number; stop: number }
	| { type: 'COLOR'; default: { r: number; g: number; b: number } }
	| { type: 'POSITION'; default: [number, number] };

export type TMapToDefaultType<T> = T extends { default: infer U } ? U : never;

type TExpandKey<GKey extends string, GValue> = GValue extends any[]
	? `${GKey}.${number}`
	: GValue extends object
		? { [P in keyof GValue]: `${GKey}.${P & string}` }[keyof GValue]
		: GKey;

export type TFieldData<GKey extends string, GInputType extends TModificationInputType> = {
	[key in GKey]: TMapToDefaultType<GInputType>;
};
