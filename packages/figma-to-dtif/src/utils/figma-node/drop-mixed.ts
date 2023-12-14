import { MixedNotSupportedException } from '../../exceptions';

export function dropMixed<GNode extends SceneNode, GNodeKeys extends keyof GNode>(
	node: GNode,
	property: GNodeKeys
): TExcludeMixed<GNode[GNodeKeys]> {
	const value = node[property];
	if (value === figma.mixed) {
		throw new MixedNotSupportedException(property.toLocaleString(), node);
	}
	return value as TExcludeMixed<GNode[GNodeKeys]>;
}

type TExcludeMixed<T> = T extends PluginAPI['mixed'] ? never : T;
