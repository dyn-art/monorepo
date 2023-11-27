import React from 'react';
import { Composition } from '@dyn/svg-composition';

import { useInteractionMode } from '../../useInteractionMode';
import { EHandleSide } from './components/CanvasControl/components/SelectionBox/components';

export function useCursorStyle(composition?: Composition) {
	const interactionMode = useInteractionMode(composition);
	return React.useMemo(() => {
		console.log({ interactionMode });
		switch (interactionMode.type) {
			case 'Resizing':
				return getCursorForInteractionMode(interactionMode.corner);
			default:
				return 'default';
		}
	}, [interactionMode]);
}

function getCursorForInteractionMode(corner: number): string {
	switch (corner) {
		case EHandleSide.Top + EHandleSide.Left:
		case EHandleSide.Bottom + EHandleSide.Right:
			return 'nwse-resize';
		case EHandleSide.Top:
		case EHandleSide.Bottom:
			return 'ns-resize';
		case EHandleSide.Top + EHandleSide.Right:
		case EHandleSide.Bottom + EHandleSide.Left:
			return 'nesw-resize';
		case EHandleSide.Right:
		case EHandleSide.Left:
			return 'ew-resize';
		default:
			return 'default';
	}
}
