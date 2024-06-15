import type { ARB } from '@dyn/arb-dtif';

export function mapFigmaLayoutElementSizingModeToDtif(
	sizingMode: 'FIXED' | 'HUG' | 'FILL'
): ARB.LayoutElementSizingMode {
	switch (sizingMode) {
		case 'FIXED':
		case 'HUG':
			return 'Fixed';
		case 'FILL':
			return 'Fill';
	}
}
