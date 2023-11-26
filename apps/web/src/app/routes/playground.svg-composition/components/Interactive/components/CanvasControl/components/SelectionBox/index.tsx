import React from 'react';
import { Composition } from '@dyn/svg-composition';

import { useInteractionMode } from '../../../../useInteractionMode';
import { useSelectedNodes } from '../../../../useSelectedNodes';
import { EHandleSide, InnerSelectionBox, TXYWH } from './components/InnerSelectionBox';

export const SelectionBox: React.FC<TProps> = (props) => {
	const { composition, onResizeHandlePointerDown } = props;
	const selectedEntities = useSelectedNodes(composition);
	const interactionMode = useInteractionMode(composition);

	return (
		<>
			{selectedEntities.map((selectedEntity) => (
				<InnerSelectionBox
					key={selectedEntity}
					composition={composition}
					entity={selectedEntity}
					showHandles={interactionMode !== 'Translating'}
					onResizeHandlePointerDown={onResizeHandlePointerDown}
				/>
			))}
		</>
	);
};

type TProps = {
	composition: Composition;
	onResizeHandlePointerDown: (corner: EHandleSide, initialBounds: TXYWH) => void;
};
