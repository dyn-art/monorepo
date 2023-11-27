import React from 'react';
import { Composition, Vec2, XYWH } from '@dyn/svg-composition';

import { useInteractionMode } from '../../../../../../useInteractionMode';
import { useSelectedNodes } from '../../../../../../useSelectedNodes';
import { EHandleSide, InnerSelectionBox } from './components/InnerSelectionBox';

export const SelectionBox: React.FC<TProps> = (props) => {
	const { composition, onResizeHandlePointerDown, onResizeHandlePointerUp } = props;
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
				/>
			))}
		</>
	);
};

type TProps = {
	composition: Composition;
	onResizeHandlePointerDown: (corner: EHandleSide, initialBounds: XYWH) => void;
	onResizeHandlePointerUp: (position: Vec2) => void;
};
