export type TPaint = TSolidPaint | TGradientPaint;

export interface TSolidPaint {
	type: 'Solid';
	color: TColor;
}

export interface TGradientPaint {
	type: 'Gradient';
	variant: TGradientVariant;
	stops: TGradientColorStop[];
}

export type TColor = TRGB;

export type TRGB = [number, number, number];

export interface TGradientColorStop {
	/**
	 * The position of the color stop in the gradient, ranging from 0.0 to 1.0.
	 */
	position: number;
	/**
	 * The color of the stop.
	 */
	color: TRGB;
	/**
	 * The opacity of the stop.
	 */
	opacity?: number;
}

export type TGradientVariant =
	| { type: 'Linear'; transform?: TMat3 }
	| { type: 'Radial'; transform?: TMat3 };

export type TMat3 = [number, number, number, number, number, number, number, number, number];
