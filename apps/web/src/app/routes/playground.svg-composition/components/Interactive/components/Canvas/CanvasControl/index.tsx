import React from 'react';
import { Composition } from '@dyn/svg-composition';

import { SelectionBox } from './components';

export const CanvasControl: React.FC<TProps> = (props) => {
	const { composition } = props;

	return (
		<svg
			id={'canvas-control'}
			className="pointer-events-none absolute h-full w-full"
			overflow={'visible'}
			style={{ cursor: 'crosshair' }}
		>
			<SelectionBox
				composition={composition}
				onResizeHandlePointerDown={(corner, initialBounds, rotationInRadians) => {
					composition.emitInteractionEvents([
						{
							type: 'CursorDownOnResizeHandle',
							corner,
							initialBounds,
							rotationInRadians: rotationInRadians
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

type TProps = {
	composition: Composition;
};
