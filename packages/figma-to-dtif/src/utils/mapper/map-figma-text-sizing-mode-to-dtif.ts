import type { ARB } from '@dyn/arb-dtif';

export function mapFigmaTextSizingModeToDtif(
	sizingMode: 'NONE' | 'WIDTH_AND_HEIGHT' | 'HEIGHT' | 'TRUNCATE'
): ARB.TextSizingMode {
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
