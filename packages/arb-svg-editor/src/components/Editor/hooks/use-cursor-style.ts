import React from 'react';
import type { Artboard } from '@dyn/arb-svg-builder';

import { CURSOR } from '../cursor';
import { useCursor } from './use-cursor';

export function useCursorStyle(canvas?: Artboard): string {
	const cursor = useCursor(canvas);

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
