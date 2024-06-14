import type { ARB } from '@dyn/arb-dtif';

export function mapFigmaLayoutParentSizingModeToDtif(
	sizingMode: 'FIXED' | 'HUG' | 'FILL'
): ARB.LayoutParentSizingMode {
	switch (sizingMode) {
		case 'FIXED':
		case 'FILL':
			return 'Fixed';
		case 'HUG':
			return 'Hug';
	}
}
