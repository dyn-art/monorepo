import type { COMP } from '@dyn/dtif-comp';

export function mapFigmaRGBToDtif(rgb: RGB): COMP.Color {
	return {
		red: Math.round(rgb.r * 255),
		green: Math.round(rgb.g * 255),
		blue: Math.round(rgb.b * 255)
	};
}
