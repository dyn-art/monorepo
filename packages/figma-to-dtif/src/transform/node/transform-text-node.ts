import type { COMP } from '@dyn/dtif-comp';

import type {
	TToTransformFill,
	TToTransformStroke,
	TToTransformTextNode
} from '../../FigmaNodeTreeProcessor';
import {
	createDtifStyles,
	mapFigmaBlendModeToDtif,
	mapFigmaHorizontalTextAlignmentToDtif,
	mapFigmaTransformToRotation,
	mapFigmaTransformToTranslation,
	mapFigmaVerticalTextAlignmentToDtif
} from '../../utils';

export function transformTextNode(
	node: TextNode,
	segments: TToTransformTextNode['segments'],
	config: TTransformTextNodeNodeConfig
): { type: 'Text' } & COMP.TextNode {
	const { fills, strokes } = config;

	return {
		type: 'Text',
		spans: segments.map(
			(segment) =>
				({
					text: segment.characters,
					font: segment.fontMetadata,
					style: {
						fontSize: segment.fontSize,
						letterSpacing:
							segment.letterSpacing.unit === 'PIXELS'
								? { Fixed: { type: 'Pixels', pixels: segment.letterSpacing.value } }
								: { Fixed: { type: 'Percent', percent: segment.letterSpacing.value } },
						lineHeight:
							// eslint-disable-next-line no-nested-ternary -- Readable enough
							segment.lineHeight.unit === 'PIXELS'
								? { Fixed: { type: 'Pixels', pixels: segment.lineHeight.value } }
								: segment.lineHeight.unit === 'PERCENT'
									? { Fixed: { type: 'Percent', percent: segment.lineHeight.value } }
									: 'Auto'
					}
				}) as COMP.TextSpan
		),
		horizontalTextAlignment: mapFigmaHorizontalTextAlignmentToDtif(node.textAlignHorizontal),
		verticalTextAlignment: mapFigmaVerticalTextAlignmentToDtif(node.textAlignVertical),
		linebreakBehavior: 'WordBoundary',
		visible: node.visible,
		size: [node.width, node.height],
		translation: mapFigmaTransformToTranslation(node.relativeTransform),
		rotationDeg: mapFigmaTransformToRotation(node.relativeTransform),
		blendMode: mapFigmaBlendModeToDtif(node.blendMode),
		opacity: node.opacity,
		styles: createDtifStyles(fills, strokes)
	};
}

interface TTransformTextNodeNodeConfig {
	fills: TToTransformFill[];
	strokes: TToTransformStroke[];
}
