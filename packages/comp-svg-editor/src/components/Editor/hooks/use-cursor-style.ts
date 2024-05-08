import React from 'react';
import type { Composition } from '@dyn/comp-svg-builder';

import { CURSOR } from '../cursor';
import { useCursor } from './use-cursor';

export function useCursorStyle(composition?: Composition): string {
	const cursor = useCursor(composition);

	return React.useMemo(() => {
		switch (cursor.type) {
			case 'Default':
				return CURSOR.default();
			case 'Grabbing':
				return CURSOR.grabbing();
			case 'Crosshair':
				return CURSOR.crosshair();
			case 'Resize':
				return CURSOR.resize(cursor.rotationDeg);
			case 'Rotate':
				return CURSOR.rotate(cursor.rotationDeg);
		}
	}, [cursor]);
}
