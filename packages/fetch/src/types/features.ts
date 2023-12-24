import type { TUnionToIntersection } from '@dyn/types/utility';

import type { TApiFeature } from './api';
import type { TOpenApiFeature } from './openapi';

export interface TFeatures<GPaths extends {} = {}> {
	base: { _: null }; // TODO: Placeholder Feature: Figure out how to make the TS infer work with [] (empty array -> no feature)
	api: TApiFeature;
	openapi: TOpenApiFeature<GPaths>;
}

export type TFeatureKeys<GPaths extends {} = {}> = keyof TFeatures<GPaths>;

export type TSelectFeatureObjects<
	GPaths extends {},
	GSelectedFeatureKeys extends TFeatureKeys<GPaths>[]
> = {
	[K in GSelectedFeatureKeys[number]]: TFeatures<GPaths>[K];
};

export type TSelectFeatures<
	GPaths extends {},
	GSelectedFeatureKeys extends TFeatureKeys<GPaths>[],
	GSelectedFeatureObjects extends TSelectFeatureObjects<
		GPaths,
		GSelectedFeatureKeys
	> = TSelectFeatureObjects<GPaths, GSelectedFeatureKeys>
> = TUnionToIntersection<GSelectedFeatureObjects[keyof GSelectedFeatureObjects]>;

export type TEnforceFeatures<
	GFeatureKeys extends TFeatureKeys[],
	GToEnforceFeatureKeys extends TFeatureKeys[]
> = Exclude<GToEnforceFeatureKeys, GFeatureKeys> extends never
	? GFeatureKeys
	: GFeatureKeys | Exclude<GToEnforceFeatureKeys, GFeatureKeys>;
