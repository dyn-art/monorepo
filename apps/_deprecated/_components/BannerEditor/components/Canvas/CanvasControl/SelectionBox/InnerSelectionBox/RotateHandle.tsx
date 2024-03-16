import React from 'react';
import type { COMP } from '@dyn/svg-composition';

export const RotateHandle: React.FC<TRotateHandleProps> = (props) => {
	const { width, height, offset, cursor, onPointerDown, onPointerUp } = props;

	return (
		<rect
			className="stroke-0"
			height={height}
			onPointerDown={onPointerDown}
			onPointerUp={onPointerUp}
			rx={360}
			ry={360}
			style={{ cursor, fill: 'rgba(0, 0, 0, 0)' }}
			width={width}
			x={offset[0]}
			y={offset[1]}
		/>
	);
};

export interface TRotateHandleProps {
	width: number;
	height: number;
	offset: COMP.Vec2;
	cursor: string;
	onPointerDown: (e: React.PointerEvent<SVGElement>) => void;
	onPointerUp: (e: React.PointerEvent<SVGElement>) => void;
}
