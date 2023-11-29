import React from 'react';
import { Composition, Vec2, XYWH } from '@dyn/svg-composition';

import { useInteractionMode, useSelectedNodes } from '../../../../../hooks';
import { InnerSelectionBox } from './InnerSelectionBox';
import { EHandleSide } from './InnerSelectionBox/controller';

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
					key={selectedEntity}
					composition={composition}
					entity={selectedEntity}
					showHandles={interactionMode.type !== 'Translating'}
					onResizeHandlePointerDown={onResizeHandlePointerDown}
					onResizeHandlePointerUp={onResizeHandlePointerUp}
					onRotateHandlePointerDown={onRotateHandlePointerDown}
					onRotateHandlePointerUp={onRotateHandlePointerUp}
				/>
			))}
		</>
	);
};

type TProps = {
	composition: Composition;
	onResizeHandlePointerDown: (corner: EHandleSide, initialBounds: XYWH, rotation: number) => void;
	onResizeHandlePointerUp: (position: Vec2) => void;
	onRotateHandlePointerDown: (corner: EHandleSide, rotation: number) => void;
	onRotateHandlePointerUp: (position: Vec2) => void;
};
