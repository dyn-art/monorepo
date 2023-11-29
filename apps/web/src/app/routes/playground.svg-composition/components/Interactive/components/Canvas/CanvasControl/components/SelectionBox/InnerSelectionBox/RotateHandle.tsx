import React from 'react';
import { Vec2 } from '@dyn/svg-composition';

export const RotateHandle: React.FC<TRotateHandleProps> = (props) => {
	const { size, offset, cursor, onPointerDown, onPointerUp } = props;

	return (
		<rect
			x={offset[0]}
			y={offset[1]}
			width={size}
			height={size}
			className="stroke-0"
			style={{ cursor, fill: 'rgba(0, 0, 0, 0)' }}
			onPointerDown={onPointerDown}
			onPointerUp={onPointerUp}
		/>
	);
};

export type TRotateHandleProps = {
	size: number;
	offset: Vec2;
	cursor: string;
	onPointerDown: (e: React.PointerEvent<SVGElement>) => void;
	onPointerUp: (e: React.PointerEvent<SVGElement>) => void;
};
