import type { CNV } from '@dyn/cnv-dtif';

export function mapFigmaRGBToDtif(rgb: RGB): CNV.Color {
	return [Math.round(rgb.r * 255), Math.round(rgb.g * 255), Math.round(rgb.b * 255)];
}
