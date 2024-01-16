import React from 'react';

export const ResizeHandle: React.FC<TResizeHandleProps> = (props) => {
	const { width, height, pointerAreaOffset = 5, cursor, onPointerDown, onPointerUp } = props;
	return (
		<>
			<rect
				className="fill-white stroke-blue-400 stroke-1"
				height={height}
				rx={4}
				ry={4}
				width={width}
			/>
			<rect
				className="stroke-0"
				height={height + pointerAreaOffset * 2}
				onPointerDown={onPointerDown}
				onPointerUp={onPointerUp}
				rx={360}
				ry={360}
				style={{ cursor, fill: 'rgba(0, 0, 0, 0)' }}
				width={width + pointerAreaOffset * 2}
				x={-pointerAreaOffset}
				y={-pointerAreaOffset}
			/>
		</>
	);
};

export interface TResizeHandleProps {
	width: number;
	height: number;
	pointerAreaOffset?: number;
	cursor: string;
	onPointerDown: (e: React.PointerEvent<SVGElement>) => void;
	onPointerUp: (e: React.PointerEvent<SVGElement>) => void;
}
