import type { TColor } from './types';

export function mapColorToCss(color: TColor): string {
	const [r, g, b] = color;
	return `rgb(${r}, ${g}, ${b})`;
}
