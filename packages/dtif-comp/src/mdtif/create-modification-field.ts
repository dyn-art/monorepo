import type { TModificationField, TModificationInputType } from './types';

export function createModificationField<
	GKey extends string,
	GInputType extends TModificationInputType,
	GInferredKey extends GKey
>(
	field: TModificationField<GKey, GInputType, GInferredKey>
): TModificationField<GKey, GInputType, GInferredKey> {
	return field;
}
