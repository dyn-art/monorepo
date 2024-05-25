import type { COMP } from '@dyn/comp-dtif';

export function mapFigmaTextSizingModeToDtif(
	sizingMode: 'NONE' | 'WIDTH_AND_HEIGHT' | 'HEIGHT' | 'TRUNCATE'
): COMP.TextSizingMode {
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
