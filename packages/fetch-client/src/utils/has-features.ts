import type { TFeatureKeys, TFetchClient } from '../types';

export function hasFeatures<
	GFeatureKeys extends TFeatureKeys[],
	GHasFeatureKeys extends TFeatureKeys[]
>(
	obj: TFetchClient<GFeatureKeys>,
	features: GHasFeatureKeys
): obj is TFetchClient<GHasFeatureKeys> {
	return features.every((feature) => feature in obj);
}
