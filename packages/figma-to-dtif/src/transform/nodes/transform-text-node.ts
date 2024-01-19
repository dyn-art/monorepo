import type { COMP } from '@dyn/dtif';

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
): { type: 'Text' } & COMP.TextNodeBundle {
	const { paintIds } = config;

	return {
		type: 'Text',
		node: {
			node_type: 'Text'
		},
		name: node.name,
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
				}) as COMP.TextSegment
		),
		horizontalTextAlignment: convertFigmaHorizontalTextAlignmentToDTIF(node.textAlignHorizontal),
		verticalTextAlignment: convertFigmaVerticalTextAlignmentToDTIF(node.textAlignVertical),
		linebreakBehavior: 'WordBoundary',
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
