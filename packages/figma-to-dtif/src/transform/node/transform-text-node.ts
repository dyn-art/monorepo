import type { COMP } from '@dyn/dtif-comp';

import type {
	TToTransformFill,
	TToTransformStroke,
	TToTransformTextNode
} from '../../FigmaNodeTreeProcessor';
import {
	createDtifStyles,
	mapFigmaBlendModeToDtif,
	mapFigmaTransformToRotation,
	mapFigmaTransformToTranslation
} from '../../utils';

export function transformTextNode(
	node: TextNode,
	attributes: TToTransformTextNode['attributes'],
	config: TTransformTextNodeNodeConfig
): { type: 'Text' } & COMP.TextNode {
	const { fills, strokes } = config;

	let nextStart = 0;

	return {
		type: 'Text',
		text: node.characters,
		attributes: attributes.map((attribute) => {
			// Figma works on char and not byte level for their ranges.
			// e.g. for "·" the end is 1 although it should be 2 on byte level.
			// Thus we need to map from char to byte level.
			let currentStart = nextStart === 0 ? attribute.start : nextStart;
			nextStart = nextStart + bytes(attribute.characters);

			return {
				start: currentStart,
				end: nextStart,
				attributes: {
					fontFamily: attribute.fontInfo.family,
					fontStretch: attribute.fontInfo.variant.stretch,
					fontStyle: attribute.fontInfo.variant.style,
					fontWeight: attribute.fontInfo.variant.weight,
					fontSize: attribute.fontSize
				}
			};
		}),
		lineWrap: 'Word',
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

// Custom implementation because "TextEncoder", "Blob", "Buffer" is ofc not supported in Figma Plugin environment
// https://stackoverflow.com/questions/2219526/how-many-bytes-in-a-javascript-string
function bytes(str: String) {
	let bytes = 0,
		len = str.length,
		codePoint,
		next,
		i;

	for (i = 0; i < len; i++) {
		codePoint = str.charCodeAt(i);

		// Lone surrogates cannot be passed to encodeURI
		if (codePoint >= 0xd800 && codePoint < 0xe000) {
			if (codePoint < 0xdc00 && i + 1 < len) {
				next = str.charCodeAt(i + 1);

				if (next >= 0xdc00 && next < 0xe000) {
					bytes += 4;
					i++;
					continue;
				}
			}
		}

		bytes += codePoint < 0x80 ? 1 : codePoint < 0x800 ? 2 : 3;
	}

	return bytes;
}
