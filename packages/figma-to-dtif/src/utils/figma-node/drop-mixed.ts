import { MixedNotSupportedException } from '../../exceptions';

export function dropMixed<GNode extends SceneNode, GNodeKeys extends keyof GNode>(
	node: GNode,
	property: GNodeKeys,
	replacement?: TExcludeMixed<GNode[GNodeKeys]>
): TExcludeMixed<GNode[GNodeKeys]> {
	const value = node[property];
	if (value === figma.mixed) {
		if (replacement != null) {
			console.warn(`Replaced mixed value of property '${property.toString()}' with: `, replacement);
			return replacement;
		}
		throw new MixedNotSupportedException(property.toLocaleString(), node);
	}
	return value as TExcludeMixed<GNode[GNodeKeys]>;
}

type TExcludeMixed<T> = T extends PluginAPI['mixed'] ? never : T;
