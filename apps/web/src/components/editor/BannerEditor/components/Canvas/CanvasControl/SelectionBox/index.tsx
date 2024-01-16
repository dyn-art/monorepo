import React from 'react';
import type { Composition, Vec2, XYWH } from '@dyn/svg-composition';

import { useInteractionMode, useSelectedNodes } from '../../../../hooks';
import { InnerSelectionBox } from './InnerSelectionBox';
import type { EHandleSide } from './InnerSelectionBox/controller';

export const SelectionBox: React.FC<TProps> = (props) => {
	const {
		composition,
		onResizeHandlePointerDown,
		onResizeHandlePointerUp,
		onRotateHandlePointerDown,
		onRotateHandlePointerUp
	} = props;
	const selectedEntities = useSelectedNodes(composition);
	const interactionMode = useInteractionMode(composition);

	return (
		<>
			{selectedEntities.map((selectedEntity) => (
				<InnerSelectionBox
					composition={composition}
					entity={selectedEntity}
					key={selectedEntity}
					onResizeHandlePointerDown={onResizeHandlePointerDown}
					onResizeHandlePointerUp={onResizeHandlePointerUp}
					onRotateHandlePointerDown={onRotateHandlePointerDown}
					onRotateHandlePointerUp={onRotateHandlePointerUp}
					showHandles={
						interactionMode.type !== 'Translating' && interactionMode.type !== 'Rotating'
					}
				/>
			))}
		</>
	);
};

interface TProps {
	composition: Composition;
	onResizeHandlePointerDown: (
		corner: EHandleSide,
		initialBounds: XYWH,
		rotationInRadians: number
	) => void;
	onResizeHandlePointerUp: (position: Vec2) => void;
	onRotateHandlePointerDown: (corner: EHandleSide, rotationInRadians: number) => void;
	onRotateHandlePointerUp: (position: Vec2) => void;
}
