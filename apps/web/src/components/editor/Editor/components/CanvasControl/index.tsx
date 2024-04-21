'use client';

import React from 'react';
import type { Composition } from '@dyn/svg-comp';

import { useSelectedEntities, useWatchComposition } from '../../hooks';
import { EntitySelectionBox } from './EntitySelectionBox';

export const CanvasControl: React.FC<TProps> = (props) => {
	const composition = useWatchComposition(props.composition, true);
	const selectedEntities = useSelectedEntities(composition);

	return (
		<svg
			className="pointer-events-none absolute left-0 top-0"
			height={composition.size[1]}
			id="canvas-control"
			overflow="visible"
			viewBox={`0 0 ${composition.size[0]} ${composition.size[1]}`}
			width={composition.size[0]}
		>
			<defs>
				<clipPath id="canvas-clip">
					<rect height={composition.size[1]} width={composition.size[0]} x={0} y={0} />
				</clipPath>
			</defs>
			<g className="pointer-events-auto" clipPath="url(#canvas-clip)">
				<g id="entity-selection">
					{selectedEntities.map((selectedEntity) => (
						<EntitySelectionBox
							composition={composition}
							entity={selectedEntity}
							key={selectedEntity}
							onResizeHandlePointerDown={(corner, initialBounds, rotationDeg) => {
								composition.emitInputEvent('Interaction', {
									type: 'CursorDownOnResizeHandle',
									corner,
									initialBounds,
									rotationRad: rotationDeg * (Math.PI / 180)
								});
							}}
							onResizeHandlePointerUp={(position) => {
								composition.emitInputEvent('Interaction', {
									type: 'CursorUpOnComposition',
									position,
									button: 'Left'
								});
							}}
							onRotateHandlePointerDown={(corner, rotationDeg) => {
								composition.emitInputEvent('Interaction', {
									type: 'CursorDownOnRotateHandle',
									corner,
									initialRotationRad: rotationDeg * (Math.PI / 180)
								});
							}}
							onRotateHandlePointerUp={(position) => {
								composition.emitInputEvent('Interaction', {
									type: 'CursorUpOnComposition',
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
	composition: Composition;
}
