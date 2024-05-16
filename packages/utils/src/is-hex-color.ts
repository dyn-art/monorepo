import type { THexColor } from './types';

export function isHexColor(color: unknown): color is THexColor {
	return typeof color === 'string' && /^#(?:[a-f\d]{3}|[a-f\d]{6})$/i.test(color);
}
