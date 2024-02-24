import type { COMP } from '../comp';
import { calculateViewport, type TDimensions } from './calculate-viewport';

export function applyCanvasDimensions(
	dtif: COMP.CompDtif,
	canvasDimensions: TDimensions
): COMP.CompDtif {
	dtif.viewport = calculateViewport(canvasDimensions, {
		width: dtif.size[0],
		height: dtif.size[1]
	});
	dtif.size = [canvasDimensions.width, canvasDimensions.height];
	return dtif;
}
