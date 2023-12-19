import type { TTextNodeBundle, TTextSegment } from '@dyn/dtif';

import type { TToTransformTextNode } from '../../FigmaNodeTreeProcessor';
import {
	convertFigmaBlendModeToDTIF,
	convertFigmaHorizontalTextAlignmentToDTIF,
	convertFigmaTransformToMat3,
	convertFigmaVerticalTextAlignmentToDTIF
} from '../../utils';

export function transformTextNode(
	node: TextNode,
	segments: TToTransformTextNode['segments'],
	config: TTransformTextNodeNodeConfig
): { type: 'Text' } & TTextNodeBundle {
	const { paintIds } = config;

	return {
		type: 'Text',
		node: {
			name: node.name,
			node_type: 'Text'
		},
		text: {
			segments: segments.map(
				(segment) =>
					({
						value: segment.characters,
						style: {
							fontId: segment.fontId,
							fontSize: segment.fontSize,
							letterSpacing:
								segment.letterSpacing.unit === 'PIXELS'
									? { Pixels: segment.letterSpacing.value }
									: { Percent: segment.letterSpacing.value },
							lineHeight:
								// eslint-disable-next-line no-nested-ternary -- Readable enough
								segment.lineHeight.unit === 'PIXELS'
									? { Pixels: segment.lineHeight.value }
									: segment.lineHeight.unit === 'PERCENT'
									? { Percent: segment.lineHeight.value }
									: 'Auto'
						}
					}) as TTextSegment
			),
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
	paintIds: number[];
}
