import React from 'react';
import { Composition } from '@dyn/svg-composition';

import { useSelectedNodes } from '../../useSelectedNodes';
import { EHandleSide, InnerSelectionBox, TXYWH } from './InnerSelectionBox';

export const SelectionBox: React.FC<TProps> = (props) => {
	const { composition, onResizeHandlePointerDown } = props;
	const selectedEntities = useSelectedNodes(composition);

	// TODO: Hide handles when translating

	return (
		<>
			{selectedEntities.map((selectedEntity) => (
				<InnerSelectionBox
					key={selectedEntity}
					composition={composition}
					entity={selectedEntity}
					showHandles={true}
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
