import type { TUnionToIntersection } from '@dyn/types/utility';

import type { TApiDelete, TApiGet, TApiPost, TApiPut } from './api';

export interface TFeatures {
	api: {
		get: TApiGet;
		put: TApiPut;
		post: TApiPost;
		del: TApiDelete;
	};
	openapi: {
		// TODO
	};
}

export type TFeatureKeys = keyof TFeatures;

export type TSelectFeatureObjects<GSelectedFeatureKeys extends TFeatureKeys[]> = {
	[K in GSelectedFeatureKeys[number]]: TFeatures[K];
};

export type TSelectFeatures<
	GSelectedFeatureKeys extends TFeatureKeys[],
	GSelectedFeatureObjects extends
		TSelectFeatureObjects<GSelectedFeatureKeys> = TSelectFeatureObjects<GSelectedFeatureKeys>
> = TUnionToIntersection<GSelectedFeatureObjects[keyof GSelectedFeatureObjects]>;

export type TEnforceFeatures<
	GFeatureKeys extends TFeatureKeys[],
	GToEnforceFeatureKeys extends TFeatureKeys[]
> = Exclude<GToEnforceFeatureKeys, GFeatureKeys> extends never
	? GFeatureKeys
	: GFeatureKeys | Exclude<GToEnforceFeatureKeys, GFeatureKeys>;
