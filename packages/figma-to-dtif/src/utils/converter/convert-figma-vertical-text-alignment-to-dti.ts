import type { TVerticalTextAlignment } from '@dyn/dtif';

export function convertFigmaVerticalTextAlignmentToDTIF(
	figmaBlendMode: TextNode['textAlignVertical']
): TVerticalTextAlignment {
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
