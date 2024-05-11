import type { COMP } from '@dyn/comp-dtif';

export function mapFigmaLayoutElementSizingModeToDtif(
	sizingMode: 'FIXED' | 'HUG' | 'FILL'
): COMP.LayoutElementSizingMode {
	switch (sizingMode) {
		case 'FIXED':
		case 'HUG':
			return 'Fixed';
		case 'FILL':
			return 'Fill';
	}
}
