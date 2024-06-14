import type { CNV } from '@dyn/cnv-dtif';

export function mapFigmaTextAlignVerticalToDtif(
	align: TextNode['textAlignVertical']
): CNV.VerticalTextAlignment {
	switch (align) {
		case 'TOP':
			return 'Top';
		case 'BOTTOM':
			return 'Bottom';
		case 'CENTER':
			return 'Center';
	}
}
