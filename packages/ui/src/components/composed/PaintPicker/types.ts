import type { TMat3, TRgbaColor } from '@ibg/utils';

export type TPaint = TSolidPaint | TGradientPaint | TImagePaint;

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

export interface TImagePaint {
	type: 'Image';
	scaleMode: TImageScaleMode;
	content?: number[];
	opacity: number;
}

export type TImageScaleMode =
	| { type: 'Fill' }
	| { type: 'Fit' }
	| { type: 'Crop'; transform?: TMat3 }
	| { type: 'Tile'; rotation?: number; scalingFactor: number };
