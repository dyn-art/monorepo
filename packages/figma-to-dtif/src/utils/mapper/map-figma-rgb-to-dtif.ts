import type { COMP } from '@dyn/comp-dtif';

export function mapFigmaRGBToDtif(rgb: RGB): COMP.Color {
	return {
		red: rgb.r * 255,
		green: rgb.g * 255,
		blue: rgb.b * 255
	};
}
