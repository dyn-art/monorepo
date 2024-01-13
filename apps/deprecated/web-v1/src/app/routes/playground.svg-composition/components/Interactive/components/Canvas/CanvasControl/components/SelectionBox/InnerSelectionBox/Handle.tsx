import React from 'react';
import { Vec2 } from '@dyn/svg-composition';

import { ResizeHandle, TResizeHandleProps } from './ResizeHandle';
import { RotateHandle, TRotateHandleProps } from './RotateHandle';

export const Handle: React.FC<TProps> = (props) => {
	const { position, pointerEvents, resizeHandle, rotateHandle = false } = props;

	return (
		<g
			key={'resize-handler'}
			style={{
				transform: `translate(${position[0]}px, ${position[1]}px)`,
				pointerEvents
			}}
		>
			<ResizeHandle {...resizeHandle} />
			{rotateHandle && <RotateHandle {...rotateHandle} />}
		</g>
	);
};

type TProps = {
	position: Vec2;
	pointerEvents: 'auto' | 'none';
	resizeHandle: TResizeHandleProps;
	rotateHandle?: TRotateHandleProps | false;
};
