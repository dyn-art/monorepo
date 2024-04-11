import React from 'react';
import type { Composition } from '@dyn/svg-comp';

import { CURSOR } from '../cursor';
import { useCursor } from './use-cursor';

export function useCursorStyle(composition?: Composition): string {
	const cursor = useCursor(composition);

	return React.useMemo(() => {
		switch (cursor.type) {
			case 'Resize':
				return CURSOR.resize(cursor.rotationDeg);
			case 'Rotate':
				return CURSOR.rotate(cursor.rotationDeg);
			case 'Default':
				return CURSOR.default();
		}
	}, [cursor]);
}
