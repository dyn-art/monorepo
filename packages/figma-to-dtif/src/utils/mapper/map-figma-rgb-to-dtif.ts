import type { COMP } from '@dyn/comp-dtif';

export function mapFigmaRGBToDtif(rgb: RGB): COMP.Color {
	return [Math.round(rgb.r * 255), Math.round(rgb.g * 255), Math.round(rgb.b * 255)];
}
