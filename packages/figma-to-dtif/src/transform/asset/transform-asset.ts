import type { COMP } from '@dyn/comp-dtif';

import type { TToTransformAsset } from '../../FigmaNodeTreeProcessor';
import { transformFontAsset, type TTransformFontAssetConfig } from './transform-font';
import { transformImageAsset, type TTransformImageAssetConfig } from './transform-image';

export async function transformAsset(
	toTransformAsset: TToTransformAsset,
	config: TTransformAssetConfig
): Promise<COMP.AssetWithId> {
	const asset = toTransformAsset.asset;
	switch (asset.type) {
		case 'Image':
			return transformImageAsset(asset, toTransformAsset.nodeIds, config.image);
		case 'Font':
			return transformFontAsset(asset, toTransformAsset.nodeIds, config.font);
	}
}

export interface TTransformAssetConfig {
	image: TTransformImageAssetConfig;
	font: TTransformFontAssetConfig;
}
