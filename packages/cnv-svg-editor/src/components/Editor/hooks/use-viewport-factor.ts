import React from 'react';
import type { CNV } from '@dyn/cnv-dtif';
import type { Canvas } from '@dyn/cnv-svg-builder';

export function useViewportFactor(canvas: Canvas): CNV.Vec2 {
	return React.useMemo(
		() => [
			canvas.size[0] / canvas.viewport.physicalSize[0],
			canvas.size[1] / canvas.viewport.physicalSize[1]
		],
		[canvas.size, canvas.viewport.physicalSize]
	);
}
