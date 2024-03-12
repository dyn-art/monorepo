import type { COMP } from '@dyn/comp-dtif';

export function mapFigmaVerticalTextAlignmentToDtif(
	figmaBlendMode: TextNode['textAlignVertical']
): COMP.VerticalTextAlignment {
	switch (figmaBlendMode) {
		case 'CENTER':
			return 'Center';
		case 'TOP':
			return 'Top';
		case 'BOTTOM':
			return 'Bottom';
		default:
			// Fallback for unmatched or undefined text alignment
			return 'Center';
	}
}
