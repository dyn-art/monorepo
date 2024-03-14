import React from 'react';
import type { COMP } from '@dyn/comp-dtif';
import type { Composition } from '@dyn/svg-comp';

export function useViewportFactor(composition: Composition): COMP.Vec2 {
	return React.useMemo(
		() => [
			composition.size[0] / composition.viewport.physicalSize[0],
			composition.size[1] / composition.viewport.physicalSize[1]
		],
		[composition.size, composition.viewport.physicalSize]
	);
}
