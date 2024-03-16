import type { COMP } from '@dyn/dtif-comp';

export function mapFigmaHorizontalTextAlignmentToDtif(
	figmaBlendMode: TextNode['textAlignHorizontal']
): COMP.HorizontalTextAlignment {
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
