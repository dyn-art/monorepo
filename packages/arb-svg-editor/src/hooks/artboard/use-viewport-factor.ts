import React from 'react';
import type { ARB } from '@dyn/arb-dtif';
import type { Artboard } from '@dyn/arb-svg-builder';

export function useViewportFactor(artboard: Artboard): ARB.Vec2 {
	return React.useMemo(
		() => [
			artboard.size[0] / artboard.viewport.physicalSize[0],
			artboard.size[1] / artboard.viewport.physicalSize[1]
		],
		[artboard.size, artboard.viewport.physicalSize]
	);
}
