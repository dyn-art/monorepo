import type { CNV } from '@dyn/cnv-dtif';

export function mapFigmaTextSizingModeToDtif(
	sizingMode: 'NONE' | 'WIDTH_AND_HEIGHT' | 'HEIGHT' | 'TRUNCATE'
): CNV.TextSizingMode {
	switch (sizingMode) {
		case 'WIDTH_AND_HEIGHT':
			return 'WidthAndHeight';
		case 'HEIGHT':
			return 'Height';
		case 'NONE':
		case 'TRUNCATE':
			return 'Fixed';
	}
}
