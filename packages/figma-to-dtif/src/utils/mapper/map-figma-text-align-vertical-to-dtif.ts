import type { ARB } from '@dyn/arb-dtif';

export function mapFigmaTextAlignVerticalToDtif(
	align: TextNode['textAlignVertical']
): ARB.VerticalTextAlignment {
	switch (align) {
		case 'TOP':
			return 'Top';
		case 'BOTTOM':
			return 'Bottom';
		case 'CENTER':
			return 'Center';
	}
}
