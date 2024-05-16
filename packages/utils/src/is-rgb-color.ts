import type { TRgbColor } from './types';

export function isRgbColor(color: unknown): color is TRgbColor {
	return (
		Array.isArray(color) &&
		color.length === 3 &&
		color.every((c) => Number.isInteger(c) && c >= 0 && c <= 255)
	);
}
