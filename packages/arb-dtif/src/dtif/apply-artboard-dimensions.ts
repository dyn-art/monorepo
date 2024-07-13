import type { ARB } from '../arb';
import { calculateViewport, type TDimensions } from './calculate-viewport';

export function applyArtboardDimensions(
	dtif: ARB.DtifArtboard,
	artboardDimensions: TDimensions
): ARB.DtifArtboard {
	dtif.viewport = calculateViewport(artboardDimensions, {
		width: dtif.size[0],
		height: dtif.size[1]
	});
	dtif.size = [artboardDimensions.width, artboardDimensions.height];
	return dtif;
}
