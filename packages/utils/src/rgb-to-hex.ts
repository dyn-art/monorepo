import type { TRgbColor } from './types';

export function rgbToHex(rgb: TRgbColor): string {
	return `#${rgb
		.map((x) => {
			const hex = x.toString(16);
			return hex.length === 1 ? `0${hex}` : hex;
		})
		.join('')}`;
}
