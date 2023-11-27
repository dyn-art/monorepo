import React from 'react';

export const Handle: React.FC<TProps> = (props) => {
	const { x, y, width, height, cursor, onPointerDown, onPointerUp } = props;

	return (
		<rect
			x={x}
			y={y}
			rx={4}
			ry={4}
			width={width}
			height={height}
			className={'fill-white stroke-blue-400 stroke-1'}
			style={{ cursor, pointerEvents: 'auto' }}
			onPointerDown={onPointerDown}
			onPointerUp={onPointerUp}
		/>
	);
};

type TProps = {
	x: number;
	y: number;
	width: number;
	height: number;
	cursor: string;
	onPointerDown: (e: React.PointerEvent<SVGElement>) => void;
	onPointerUp: (e: React.PointerEvent<SVGElement>) => void;
};
