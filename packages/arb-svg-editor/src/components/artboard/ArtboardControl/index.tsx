'use client';

import React from 'react';
import type { Artboard } from '@dyn/arb-svg-builder';
import { useSelectedEntities, useWatchArtboard } from '@/hooks';

import { EntitySelectionBox } from './EntitySelectionBox';

export const ArtboardControl: React.FC<TProps> = (props) => {
	const artboard = useWatchArtboard(props.artboard, true);
	const selectedEntities = useSelectedEntities(artboard);

	React.useEffect(() => {
		selectedEntities.forEach((entity) => {
			artboard.logEntityComponents(entity);
		});
	}, [selectedEntities, artboard]);

	return (
		<svg
			className="pointer-events-none absolute left-0 top-0"
			height={artboard.size[1]}
			id="artboard-control"
			overflow="visible"
			viewBox={`0 0 ${artboard.size[0].toString()} ${artboard.size[1].toString()}`}
			// Note: Not applying the viewbox of the artboard
			// because we wan't the artboard control elements
			// remain a consistent size no matter the zoom
			// viewBox={`${artboard.viewport.physicalPosition[0]} ${artboard.viewport.physicalPosition[1]} ${artboard.viewport.physicalSize[0]} ${artboard.viewport.physicalSize[1]}`}
			width={artboard.size[0]}
		>
			<defs>
				<clipPath id="artboard-clip">
					<rect height={artboard.size[1]} width={artboard.size[0]} x={0} y={0} />
				</clipPath>
			</defs>
			<g className="pointer-events-auto" clipPath="url(#artboard-clip)">
				<g id="entity-selection">
					{selectedEntities.map((selectedEntity) => (
						<EntitySelectionBox
							artboard={artboard}
							entity={selectedEntity}
							key={selectedEntity}
							onResizeHandlePointerDown={(corner, initialBounds, rotationDeg) => {
								artboard.emitInputEvent('Interaction', {
									type: 'CursorDownOnResizeHandle',
									corner,
									initialBounds,
									rotationRad: rotationDeg * (Math.PI / 180)
								});
							}}
							onResizeHandlePointerUp={(position) => {
								artboard.emitInputEvent('Interaction', {
									type: 'CursorUpOnArtboard',
									position,
									button: 'Left'
								});
							}}
							onRotateHandlePointerDown={(corner, rotationDeg) => {
								artboard.emitInputEvent('Interaction', {
									type: 'CursorDownOnRotateHandle',
									corner,
									initialRotationRad: rotationDeg * (Math.PI / 180)
								});
							}}
							onRotateHandlePointerUp={(position) => {
								artboard.emitInputEvent('Interaction', {
									type: 'CursorUpOnArtboard',
									position,
									button: 'Left'
								});
							}}
						/>
					))}
				</g>
			</g>
		</svg>
	);
};

interface TProps {
	artboard: Artboard;
}
