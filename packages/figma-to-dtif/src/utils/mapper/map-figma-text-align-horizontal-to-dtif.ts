import type { CNV } from '@dyn/cnv-dtif';

export function mapFigmaTextAlignHorizontalToDtif(
	align: TextNode['textAlignHorizontal']
): CNV.HorizontalTextAlignment {
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
