import type { TFeatureKeys, TFetchClient } from '../types';

export function withApi<GValue, GSelectedFeatureKeys extends TFeatureKeys[]>(): TFetchClient<
	['api', ...GSelectedFeatureKeys]
> {
	return null as any;
}
