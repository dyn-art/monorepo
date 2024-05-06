import type { COMP } from '@dyn/comp-dtif';

export function mapFigmaTextAlignHorizontalToDtif(
	align: TextNode['textAlignHorizontal']
): COMP.HorizontalTextAlignment {
	switch (align) {
		case 'LEFT':
			return 'Left';
		case 'RIGHT':
			return 'Right';
		case 'CENTER':
			return 'Center';
		case 'JUSTIFIED':
			return 'Left'; // TODO
	}
}
