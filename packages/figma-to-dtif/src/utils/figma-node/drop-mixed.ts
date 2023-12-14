import { Err, Ok, type Result } from 'ts-results';

import { MixedNotSupportedException } from '../../exceptions';

export function dropMixed<GNode extends SceneNode, GNodeKeys extends keyof GNode>(
	node: GNode,
	property: GNodeKeys
): Result<TExcludeMixed<GNode[GNodeKeys]>, MixedNotSupportedException> {
	const value = node[property];
	if (value === figma.mixed) {
		return Err(new MixedNotSupportedException(property.toLocaleString(), node));
	}
	return Ok(value as TExcludeMixed<GNode[GNodeKeys]>);
}

type TExcludeMixed<T> = T extends PluginAPI['mixed'] ? never : T;
