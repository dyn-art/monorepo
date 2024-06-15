import type { ARB } from '@dyn/arb-dtif';

import type { TFigmaLayoutNode } from '../../types';
import { mapFigmaConstraintsToDtif } from './map-figma-constraints-to-dtif';
import { mapFigmaLayoutElementSizingModeToDtif } from './map-figma-layout-element-sizing-mode-to-dti';

export function mapFigmaElementLayoutToDtif(
	node: TFigmaLayoutNode,
	autoLayout: boolean
): ARB.LayoutElement {
	if (autoLayout) {
		return {
			type: 'Static',
			horizontalSizingMode: mapFigmaLayoutElementSizingModeToDtif(node.layoutSizingHorizontal),
			verticalSizingMode: mapFigmaLayoutElementSizingModeToDtif(node.layoutSizingVertical)
		};
	}
	return { type: 'Absolute', constraints: mapFigmaConstraintsToDtif(node.constraints) };
}
