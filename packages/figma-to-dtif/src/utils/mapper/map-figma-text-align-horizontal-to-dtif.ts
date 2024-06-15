import type { ARB } from '@dyn/arb-dtif';

export function mapFigmaTextAlignHorizontalToDtif(
	align: TextNode['textAlignHorizontal']
): ARB.HorizontalTextAlignment {
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
