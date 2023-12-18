import type { TFeatureKeys, TState } from './types';

export function hasFeatures<
	GValue,
	GFeatureKeys extends TFeatureKeys<GValue>[],
	GHasFeatureKeys extends TFeatureKeys<GValue>[]
>(
	obj: TState<GValue, GFeatureKeys>,
	features: GHasFeatureKeys
): obj is TState<GValue, GHasFeatureKeys> {
	return features.every((feature) => feature in obj);
}
