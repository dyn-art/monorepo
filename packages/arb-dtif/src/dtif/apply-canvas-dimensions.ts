import type { ARB } from '../arb';
import { calculateViewport, type TDimensions } from './calculate-viewport';

export function applyArtboardDimensions(
	dtif: ARB.DtifArtboard,
	canvasDimensions: TDimensions
): ARB.DtifArtboard {
	dtif.viewport = calculateViewport(canvasDimensions, {
		width: dtif.size[0],
		height: dtif.size[1]
	});
	dtif.size = [canvasDimensions.width, canvasDimensions.height];
	return dtif;
}
