import type { COMP } from '@dyn/comp-dtif';

export function mapFigmaLayoutParentSizingModeToDtif(
	sizingMode: 'FIXED' | 'HUG' | 'FILL'
): COMP.LayoutParentSizingMode {
	switch (sizingMode) {
		case 'FIXED':
		case 'FILL':
			return 'Fixed';
		case 'HUG':
			return 'Hug';
	}
}
