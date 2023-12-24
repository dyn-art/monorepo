import type { TFeatureKeys, TFetchClient } from '../types';

export function withOpenApi<GSelectedFeatureKeys extends TFeatureKeys[]>(): TFetchClient<
	['openapi', ...GSelectedFeatureKeys]
> {
	return null as any;
}
