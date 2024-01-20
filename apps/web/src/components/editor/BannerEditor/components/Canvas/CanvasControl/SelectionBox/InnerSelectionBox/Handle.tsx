import React from 'react';
import type { COMP, Composition } from '@dyn/svg-composition';

import { useForwardEvents } from '../../../../../hooks';
import { ResizeHandle, type TResizeHandleProps } from './ResizeHandle';
import { RotateHandle, type TRotateHandleProps } from './RotateHandle';

export const Handle: React.FC<TProps> = (props) => {
	const { position, pointerEvents, resizeHandle, rotateHandle = false, composition } = props;
	const ref = useForwardEvents<SVGGElement>(composition, ['wheel']);

	return (
		<g
			key="resize-handler"
			ref={ref}
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
	composition: Composition;
	position: COMP.Vec2;
	pointerEvents: 'auto' | 'none';
	resizeHandle: TResizeHandleProps;
	rotateHandle?: TRotateHandleProps | false;
}
