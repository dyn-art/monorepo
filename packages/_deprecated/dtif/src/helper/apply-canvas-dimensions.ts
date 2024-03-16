import type { COMP } from '../comp';
import { calculateViewBox, type TDimensions } from './calculate-view-box';

export function applyCanvasDimensions(
	dtif: COMP.DTIFComposition,
	canvasDimensions: TDimensions
): COMP.DTIFComposition {
	dtif.viewBox = calculateViewBox(canvasDimensions, { width: dtif.width, height: dtif.height });
	dtif.width = canvasDimensions.width;
	dtif.height = canvasDimensions.height;
	return dtif;
}
