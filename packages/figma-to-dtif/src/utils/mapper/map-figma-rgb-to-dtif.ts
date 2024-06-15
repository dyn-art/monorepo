import type { ARB } from '@dyn/arb-dtif';

export function mapFigmaRGBToDtif(rgb: RGB): ARB.Color {
	return [Math.round(rgb.r * 255), Math.round(rgb.g * 255), Math.round(rgb.b * 255)];
}
