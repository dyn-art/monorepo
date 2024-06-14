import type { CNV } from '../cnv';
import { calculateViewport, type TDimensions } from './calculate-viewport';

export function applyCanvasDimensions(
	dtif: CNV.DtifCanvas,
	canvasDimensions: TDimensions
): CNV.DtifCanvas {
	dtif.viewport = calculateViewport(canvasDimensions, {
		width: dtif.size[0],
		height: dtif.size[1]
	});
	dtif.size = [canvasDimensions.width, canvasDimensions.height];
	return dtif;
}
