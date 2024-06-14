import type { CNV } from '@dyn/cnv-dtif';

export function mapFigmaLayoutParentSizingModeToDtif(
	sizingMode: 'FIXED' | 'HUG' | 'FILL'
): CNV.LayoutParentSizingMode {
	switch (sizingMode) {
		case 'FIXED':
		case 'FILL':
			return 'Fixed';
		case 'HUG':
			return 'Hug';
	}
}
