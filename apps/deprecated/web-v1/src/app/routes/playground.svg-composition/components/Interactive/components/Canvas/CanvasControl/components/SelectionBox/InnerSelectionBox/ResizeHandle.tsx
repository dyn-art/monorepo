import React from 'react';

export const ResizeHandle: React.FC<TResizeHandleProps> = (props) => {
	const { width, height, pointerAreaOffset = 5, cursor, onPointerDown, onPointerUp } = props;
	return (
		<>
			<rect
				rx={4}
				ry={4}
				width={width}
				height={height}
				className={'fill-white stroke-blue-400 stroke-1'}
			/>
			<rect
				x={-pointerAreaOffset}
				y={-pointerAreaOffset}
				rx={360}
				ry={360}
				width={width + pointerAreaOffset * 2}
				height={height + pointerAreaOffset * 2}
				className="stroke-0"
				style={{ cursor, fill: 'rgba(0, 0, 0, 0)' }}
				onPointerDown={onPointerDown}
				onPointerUp={onPointerUp}
			/>
		</>
	);
};

export type TResizeHandleProps = {
	width: number;
	height: number;
	pointerAreaOffset?: number;
	cursor: string;
	onPointerDown: (e: React.PointerEvent<SVGElement>) => void;
	onPointerUp: (e: React.PointerEvent<SVGElement>) => void;
};
