import type { THorizontalTextAlignment } from '@dyn/dtif';

export function convertFigmaHorizontalTextAlignmentToDTIF(
	figmaBlendMode: TextNode['textAlignHorizontal']
): THorizontalTextAlignment {
	switch (figmaBlendMode) {
		case 'CENTER':
			return 'Center';
		case 'LEFT':
			return 'Left';
		case 'RIGHT':
			return 'Right';
		default:
			// Fallback for unmatched or undefined text alignment
			return 'Left';
	}
}
