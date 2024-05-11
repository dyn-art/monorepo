import type { COMP } from '@dyn/comp-dtif';

import type { TFigmaLayoutNode } from '../../types';
import { mapFigmaConstraintsToDtif } from './map-figma-constraints-to-dtif';
import { mapFigmaLayoutSizingModeToDtif } from './map-figma-layout-sizing-mode-to-dtif';

export function mapFigmaElementLayoutToDtif(
	node: TFigmaLayoutNode,
	autoLayout: boolean
): COMP.LayoutElement {
	if (autoLayout) {
		return {
			type: 'Static',
			horizontalSizingMode: mapFigmaLayoutSizingModeToDtif(node.layoutSizingHorizontal),
			verticalSizingMode: mapFigmaLayoutSizingModeToDtif(node.layoutSizingVertical)
		};
	}
	return { type: 'Absolute', constraints: mapFigmaConstraintsToDtif(node.constraints) };
}
