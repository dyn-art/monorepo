import React from 'react';
import { Vec2 } from '@dyn/svg-composition';

export const RotateHandle: React.FC<TRotateHandleProps> = (props) => {
	const { width, height, offset, cursor, onPointerDown, onPointerUp } = props;

	return (
		<rect
			x={offset[0]}
			y={offset[1]}
			width={width}
			height={height}
			rx={360}
			ry={360}
			className="stroke-0"
			style={{ cursor, fill: 'rgba(0, 0, 0, 0)' }}
			onPointerDown={onPointerDown}
			onPointerUp={onPointerUp}
		/>
	);
};

export type TRotateHandleProps = {
	width: number;
	height: number;
	offset: Vec2;
	cursor: string;
	onPointerDown: (e: React.PointerEvent<SVGElement>) => void;
	onPointerUp: (e: React.PointerEvent<SVGElement>) => void;
};
