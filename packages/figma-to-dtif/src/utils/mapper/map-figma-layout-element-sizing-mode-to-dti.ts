import type { CNV } from '@dyn/cnv-dtif';

export function mapFigmaLayoutElementSizingModeToDtif(
	sizingMode: 'FIXED' | 'HUG' | 'FILL'
): CNV.LayoutElementSizingMode {
	switch (sizingMode) {
		case 'FIXED':
		case 'HUG':
			return 'Fixed';
		case 'FILL':
			return 'Fill';
	}
}
