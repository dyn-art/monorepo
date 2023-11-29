import React from 'react';
import { Composition } from '@dyn/svg-composition';

import {
	ResizeCursor,
	RotateCursor
} from '../components/Canvas/CanvasControl/components/SelectionBox/InnerSelectionBox/Cursor';
import { useCursor } from './useCursor';

export function useCursorStyle(composition?: Composition): string {
	const cursor = useCursor(composition);
	return React.useMemo(() => {
		switch (cursor.type) {
			case 'Resize':
				return ResizeCursor.constructString(cursor.rotation);
			case 'Rotate':
				return RotateCursor.constructString(cursor.rotation);
			case 'Default':
			default:
				return 'Default';
		}
	}, [cursor]);
}
