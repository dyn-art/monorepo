import type { TField, TInputType } from './types';

export function createField<
	GKey extends string,
	GInputType extends TInputType,
	GInferredKey extends GKey
>(field: TField<GKey, GInputType, GInferredKey>): TField<GKey, GInputType, GInferredKey> {
	return field;
}
