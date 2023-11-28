import React from 'react';

export const RotateHandle: React.FC<TRotateHandleProps> = (props) => {
	const { size, offset, cursor, onPointerDown, onPointerUp } = props;

	return (
		<rect
			x={-offset - size}
			y={-offset - size}
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
	offset: number;
	cursor: string;
	onPointerDown: (e: React.PointerEvent<SVGElement>) => void;
	onPointerUp: (e: React.PointerEvent<SVGElement>) => void;
};
