import type { COMP } from '../comp';

export function calculateViewport(
	svgDimensions: TDimensions,
	rectDimensions: TDimensions
): COMP.Viewport {
	const scaleX = svgDimensions.width / rectDimensions.width;
	const scaleY = svgDimensions.height / rectDimensions.height;

	// Choose the smaller scale to ensure the rectangle fits within the SVG
	const scale = Math.min(scaleX, scaleY);

	// Calculate the new dimensions of the rectangle
	const scaledWidth = rectDimensions.width * scale;
	const scaledHeight = rectDimensions.height * scale;

	// Calculate the offset to center the rectangle
	const offsetX = (svgDimensions.width - scaledWidth) / 2;
	const offsetY = (svgDimensions.height - scaledHeight) / 2;

	return {
		physicalPosition: [-offsetX / scale, -offsetY / scale],
		physicalSize: [svgDimensions.width / scale, svgDimensions.height / scale]
	};
}

export interface TDimensions {
	width: number;
	height: number;
}
