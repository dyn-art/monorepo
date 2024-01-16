import React from 'react';
import { Composition } from '@dyn/svg-composition';

import {
	ResizeCursor,
	RotateCursor
} from '../components/Canvas/CanvasControl/SelectionBox/InnerSelectionBox/Cursor';
import { useCursor } from './use-cursor';

export function useCursorStyle(composition?: Composition): string {
	const cursor = useCursor(composition);
	return React.useMemo(() => {
		switch (cursor.type) {
			case 'Resize':
				return ResizeCursor.constructString(cursor.rotationInDegrees);
			case 'Rotate':
				return RotateCursor.constructString(cursor.rotationInDegrees);
			case 'Default':
			default:
				return 'Default';
		}
	}, [cursor]);
}
