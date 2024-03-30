import type { COMP } from '@dyn/dtif-comp';

export function mapFigmaTextAlignVerticalToDtif(
	align: TextNode['textAlignVertical']
): COMP.VerticalTextAlignment {
	switch (align) {
		case 'TOP':
			return 'Top';
		case 'BOTTOM':
			return 'Bottom';
		case 'CENTER':
			return 'Center';
	}
}
