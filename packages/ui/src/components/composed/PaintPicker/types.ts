import type { TMat3, TRgbaColor } from '@dyn/utils';

export type TPaint = TSolidPaint | TGradientPaint;

export interface TSolidPaint {
	type: 'Solid';
	color: TRgbaColor;
}

export interface TGradientPaint {
	type: 'Gradient';
	variant: TGradientVariant;
	stops: TGradientColorStop[];
	opacity: number;
}

export interface TGradientColorStop {
	/**
	 * The position of the color stop in the gradient, ranging from 0.0 to 1.0.
	 */
	position: number;
	/**
	 * The color of the stop.
	 */
	color: TRgbaColor;
}

export type TGradientVariant =
	| { type: 'Linear'; transform?: TMat3 }
	| { type: 'Radial'; transform?: TMat3 };
