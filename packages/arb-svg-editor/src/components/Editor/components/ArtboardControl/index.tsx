'use client';

import React from 'react';
import type { Artboard } from '@dyn/arb-svg-builder';

import { useSelectedEntities, useWatchArtboard } from '../../hooks';
import { EntitySelectionBox } from './EntitySelectionBox';

export const ArtboardControl: React.FC<TProps> = (props) => {
	const canvas = useWatchArtboard(props.canvas, true);
	const selectedEntities = useSelectedEntities(canvas);

	React.useEffect(() => {
		selectedEntities.forEach((entity) => {
			canvas.logEntityComponents(entity);
		});
	}, [selectedEntities, canvas]);

	return (
		<svg
			className="pointer-events-none absolute left-0 top-0"
			height={canvas.size[1]}
			id="canvas-control"
			overflow="visible"
			viewBox={`0 0 ${canvas.size[0]} ${canvas.size[1]}`}
			// Note: Not applying the viewbox of the canvas
			// because we wan't the canvas control elements
			// remain a consistent size no matter the zoom
			// viewBox={`${canvas.viewport.physicalPosition[0]} ${canvas.viewport.physicalPosition[1]} ${canvas.viewport.physicalSize[0]} ${canvas.viewport.physicalSize[1]}`}
			width={canvas.size[0]}
		>
			<defs>
				<clipPath id="canvas-clip">
					<rect height={canvas.size[1]} width={canvas.size[0]} x={0} y={0} />
				</clipPath>
			</defs>
			<g className="pointer-events-auto" clipPath="url(#canvas-clip)">
				<g id="entity-selection">
					{selectedEntities.map((selectedEntity) => (
						<EntitySelectionBox
							canvas={canvas}
							entity={selectedEntity}
							key={selectedEntity}
							onResizeHandlePointerDown={(corner, initialBounds, rotationDeg) => {
								canvas.emitInputEvent('Interaction', {
									type: 'CursorDownOnResizeHandle',
									corner,
									initialBounds,
									rotationRad: rotationDeg * (Math.PI / 180)
								});
							}}
							onResizeHandlePointerUp={(position) => {
								canvas.emitInputEvent('Interaction', {
									type: 'CursorUpOnArtboard',
									position,
									button: 'Left'
								});
							}}
							onRotateHandlePointerDown={(corner, rotationDeg) => {
								canvas.emitInputEvent('Interaction', {
									type: 'CursorDownOnRotateHandle',
									corner,
									initialRotationRad: rotationDeg * (Math.PI / 180)
								});
							}}
							onRotateHandlePointerUp={(position) => {
								canvas.emitInputEvent('Interaction', {
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
	canvas: Artboard;
}
