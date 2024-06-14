import React from 'react';
import type { ARB } from '@dyn/arb-dtif';
import type { Artboard } from '@dyn/arb-svg-builder';

export function useViewportFactor(canvas: Artboard): ARB.Vec2 {
	return React.useMemo(
		() => [
			canvas.size[0] / canvas.viewport.physicalSize[0],
			canvas.size[1] / canvas.viewport.physicalSize[1]
		],
		[canvas.size, canvas.viewport.physicalSize]
	);
}
