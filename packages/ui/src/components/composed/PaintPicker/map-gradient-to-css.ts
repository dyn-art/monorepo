import { mapColorToCss } from './map-color-to-css';
import type { TGradientPaint, TMat3 } from './types';

export function mapGradientToCss(gradient: TGradientPaint): string {
	const { variant, stops } = gradient;

	// Map color stops to CSS-compatible strings
	const colorStops = stops
		.map(
			({ position, color, opacity }) =>
				`${mapColorToCss(color)}${opacity != null ? ` ${opacity}` : ''} ${position * 100}%`
		)
		.join(', ');

	switch (variant.type) {
		case 'Linear':
			return `linear-gradient(${mat3ToAngle(variant.transform)}deg, ${colorStops})`;
		case 'Radial':
			return `radial-gradient(${colorStops})`;
	}
}

function mat3ToAngle(mat3?: TMat3): number {
	if (!mat3) return 0;
	// Assuming mat3 represents a rotation matrix and we need to extract the angle
	const [a, b] = mat3;
	return Math.atan2(b, a) * (180 / Math.PI);
}
