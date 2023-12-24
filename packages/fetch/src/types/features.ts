import type { TUnionToIntersection } from '@dyn/types/utility';

import type { TApiFeature } from './api';
import type { TOpenApiFeature } from './openapi';

export interface TFeatures {
	base: { _: null }; // TODO: Placeholder Feature: Figure out how to make the TS infer work with [] (empty array -> no feature)
	api: TApiFeature;
	openapi: TOpenApiFeature;
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
