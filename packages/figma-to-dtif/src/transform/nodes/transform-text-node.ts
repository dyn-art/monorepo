import type { TTextNodeBundle } from '@dyn/dtif';

import {
	convertFigmaBlendModeToDTIF,
	convertFigmaHorizontalTextAlignmentToDTIF,
	convertFigmaTransformToMat3,
	convertFigmaVerticalTextAlignmentToDTIF,
	dropMixed
} from '../../utils';

export function transformTextNode(
	node: TextNode,
	config: TTransformTextNodeNodeConfig
): { type: 'Text' } & TTextNodeBundle {
	const { fontIds, paintIds } = config;

	const fontSize = dropMixed(node, 'fontSize');
	const letterSpacing = dropMixed(node, 'letterSpacing');
	const lineHeight = dropMixed(node, 'lineHeight');

	return {
		type: 'Text',
		node: {
			name: node.name,
			node_type: 'Text'
		},
		text: {
			// TODO: Support multipe segments
			segments: [
				{
					value: node.characters,
					style: {
						fontId: fontIds[0] as unknown as number, // TODO:
						fontSize,
						letterSpacing:
							letterSpacing.unit === 'PIXELS'
								? { Pixels: letterSpacing.value }
								: { Percent: letterSpacing.value },
						lineHeight:
							// eslint-disable-next-line no-nested-ternary -- Readable enough
							lineHeight.unit === 'PIXELS'
								? { Pixels: lineHeight.value }
								: lineHeight.unit === 'PERCENT'
								? { Percent: lineHeight.value }
								: 'Auto'
					}
				}
			],
			horizontalTextAlignment: convertFigmaHorizontalTextAlignmentToDTIF(node.textAlignHorizontal),
			verticalTextAlignment: convertFigmaVerticalTextAlignmentToDTIF(node.textAlignVertical),
			linebreakBehaviour: 'WordBoundary'
		},
		compositionMixin: {
			isLocked: node.locked,
			isVisible: node.visible
		},
		dimension: {
			height: node.height,
			width: node.width
		},
		relativeTransform: convertFigmaTransformToMat3(node.relativeTransform),
		blendMixin: {
			blendMode: convertFigmaBlendModeToDTIF(node.blendMode),
			opacity: node.opacity,
			isMask: node.isMask
		},
		fill: { paintIds }
	};
}

interface TTransformTextNodeNodeConfig {
	fontIds: number[];
	paintIds: number[];
}
