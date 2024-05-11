import type { COMP } from '@dyn/comp-dtif';

export function mapFigmaLayoutSizingModeToDtif(
	sizingMode: 'FIXED' | 'HUG' | 'FILL'
): COMP.LayoutSizingMode {
	switch (sizingMode) {
		case 'FIXED':
			return 'Fixed';
		case 'HUG':
			return 'Hug';
		case 'FILL':
			return 'Fill';
	}
}
