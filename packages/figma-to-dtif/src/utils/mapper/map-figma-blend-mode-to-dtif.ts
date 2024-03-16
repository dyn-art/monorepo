import type { COMP } from '@dyn/dtif-comp';

export function mapFigmaBlendModeToDtif(figmaBlendMode?: BlendMode): COMP.BlendMode {
	switch (figmaBlendMode) {
		case 'NORMAL':
			return 'Normal';
		case 'DARKEN':
		case 'LINEAR_BURN':
			return 'Darken';
		case 'MULTIPLY':
			return 'Multiply';
		case 'COLOR_BURN':
			return 'ColorBurn';
		case 'LIGHTEN':
		case 'LINEAR_DODGE':
			return 'Lighten';
		case 'SCREEN':
			return 'Screen';
		case 'COLOR_DODGE':
			return 'ColorDodge';
		case 'OVERLAY':
			return 'Overlay';
		case 'SOFT_LIGHT':
			return 'SoftLight';
		case 'HARD_LIGHT':
			return 'HardLight';
		case 'DIFFERENCE':
			return 'Difference';
		case 'EXCLUSION':
			return 'Exclusion';
		case 'HUE':
			return 'Hue';
		case 'SATURATION':
			return 'Saturation';
		case 'COLOR':
			return 'Color';
		case 'LUMINOSITY':
			return 'Luminosity';
		case 'PASS_THROUGH':
		default:
			// Fallback for unmatched or undefined blend modes
			return 'Normal';
	}
}
