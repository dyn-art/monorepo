import React from 'react';
import type { COMP } from '@dyn/svg-composition';

import { ResizeHandle, type TResizeHandleProps } from './ResizeHandle';
import { RotateHandle, type TRotateHandleProps } from './RotateHandle';

export const Handle: React.FC<TProps> = (props) => {
	const { position, pointerEvents, resizeHandle, rotateHandle = false } = props;

	return (
		<g
			key="resize-handler"
			style={{
				transform: `translate(${position[0]}px, ${position[1]}px)`,
				pointerEvents
			}}
		>
			<ResizeHandle {...resizeHandle} />
			{rotateHandle ? <RotateHandle {...rotateHandle} /> : null}
		</g>
	);
};

interface TProps {
	position: COMP.Vec2;
	pointerEvents: 'auto' | 'none';
	resizeHandle: TResizeHandleProps;
	rotateHandle?: TRotateHandleProps | false;
}
