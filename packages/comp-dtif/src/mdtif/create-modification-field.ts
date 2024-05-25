import type { TModificationField, TModificationInputVariant } from './types';

export function createModificationField<
	GKey extends string,
	GInputVariant extends TModificationInputVariant,
	GInferredKey extends GKey
>(
	field: TModificationField<GKey, GInputVariant, GInferredKey>
): TModificationField<GKey, GInputVariant, GInferredKey> {
	return field;
}
