import type { TVec2 } from '@ibg/utils';

import type { CNV } from '../cnv';

export interface TMdtifCanvas extends CNV.DtifCanvas {
	extension: 'MDTIF';
	scripts?: TModificationScript[];
}

export interface TModificationScript<
	GInputVariant extends TModificationInputVariant = TModificationInputVariant
> extends CNV.LuaScriptWithId {
	inputVariant: GInputVariant;
	displayName: string;
}

export type TModificationInputVariant =
	| TNumberModificationInput
	| TTextModificationInput
	| TBooleanModificationInput
	| TRangeModificationInput
	| TPaintModificationInput
	| TColorModificationInput
	| TPositionModificationInput
	| TDateTimeModificationInput;

export interface TNumberModificationInput {
	type: 'NUMBER';
	_argsMapType?: { input: number };
	default: NonNullable<TNumberModificationInput['_argsMapType']>;
	max?: number;
	min?: number;
}

export interface TTextModificationInput {
	type: 'TEXT';
	_argsMapType?: { input: string };
	default: NonNullable<TTextModificationInput['_argsMapType']>;
	area: boolean;
}

export interface TBooleanModificationInput {
	type: 'BOOLEAN';
	_argsMapType?: { input: boolean };
	default: NonNullable<TBooleanModificationInput['_argsMapType']>;
}

export interface TRangeModificationInput {
	type: 'RANGE';
	_argsMapType?: { input: number };
	default: NonNullable<TRangeModificationInput['_argsMapType']>;
	max: number;
	min: number;
	step?: number;
}

export interface TColorModificationInput {
	type: 'COLOR';
	_argsMapType?: { r: number; g: number; b: number; a: number };
	default: NonNullable<TColorModificationInput['_argsMapType']>;
}

export interface TPaintModificationInput {
	type: 'PAINT';
	_argsMapType?: TPaintModificationReturnType;
	default: NonNullable<TPaintModificationInput['_argsMapType']>;
}

export interface TPaintModificationReturnType {
	paint: CNV.Paint;
	opacity?: number;
	content?: number[];
}

export interface TPositionModificationInput {
	type: 'POSITION';
	_argsMapType?: { x: number; y: number };
	default: NonNullable<TPositionModificationInput['_argsMapType']>;
	max?: TVec2;
	min?: TVec2;
}

export interface TDateTimeModificationInput {
	type: 'DATETIME';
	_argsMapType?: { input: number };
	default: NonNullable<TDateTimeModificationInput['_argsMapType']> | 'NOW';
	withTime?: boolean;
}

export type TArgsMapType<T> = T extends { _argsMapType?: infer U } ? NonNullable<U> : never;
