import React from 'react';
import type { Composition } from '@dyn/svg-composition';

import { useWatchComposition } from '../../../hooks';
import { SelectionBox } from './SelectionBox';

export const CanvasControl: React.FC<TProps> = (props) => {
	const { composition } = props;
	useWatchComposition(composition);

	return (
		<svg
			className="pointer-events-none absolute"
			height={`${composition.height}px`}
			id="canvas-control"
			overflow="visible"
			style={{ cursor: 'crosshair' }}
			viewBox={`0 0 ${composition.width} ${composition.height}`}
			width={`${composition.width}px`}
		>
			<SelectionBox
				composition={composition}
				onResizeHandlePointerDown={(corner, initialBounds, rotationInRadians) => {
					composition.emitInteractionEvents([
						{
							type: 'CursorDownOnResizeHandle',
							corner,
							initialBounds,
							rotationInRadians
						}
					]);
				}}
				onResizeHandlePointerUp={(position) => {
					composition.emitInteractionEvents([{ type: 'CursorUpOnComposition', position }]);
				}}
				onRotateHandlePointerDown={(corner, rotationInRadians) => {
					composition.emitInteractionEvents([
						{
							type: 'CursorDownOnRotateHandle',
							corner,
							initialRotationInRadians: rotationInRadians
						}
					]);
				}}
				onRotateHandlePointerUp={(position) => {
					composition.emitInteractionEvents([{ type: 'CursorUpOnComposition', position }]);
				}}
			/>
		</svg>
	);
};

interface TProps {
	composition: Composition;
}
