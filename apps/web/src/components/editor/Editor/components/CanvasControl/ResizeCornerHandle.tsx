import React from 'react';
import type { COMP } from '@dyn/dtif-comp';

const HANDLE_SIZE = 8;
const INTERACTION_AREA_SIZE = 20;
const interactionAreaOffset = (INTERACTION_AREA_SIZE - HANDLE_SIZE) / 2;

export const ResizeCornerHandle: React.FC<ResizeCornerHandleProps> = (props) => {
	const { position, parentSize, onPointerDown, onPointerUp } = props;

	const [x, y] = React.useMemo<COMP.Vec2>(
		() => calculatePosition(position, parentSize),
		[position, parentSize]
	);

	return (
		<g id={`resize-corner-${position}`} onPointerDown={onPointerDown} onPointerUp={onPointerUp}>
			<rect
				className="fill-white stroke-blue-400 stroke-1"
				height={HANDLE_SIZE}
				width={HANDLE_SIZE}
				x={x}
				y={y}
			/>
			<rect
				className="stroke-green-500 stroke-2"
				fill="transparent"
				height={INTERACTION_AREA_SIZE}
				width={INTERACTION_AREA_SIZE}
				x={x - interactionAreaOffset}
				y={y - interactionAreaOffset}
			/>
		</g>
	);
};

function calculatePosition(position: TPosition, parentSize: COMP.Size): COMP.Vec2 {
	switch (position) {
		case 'topLeft':
			return [-HANDLE_SIZE / 2, -HANDLE_SIZE / 2];
		case 'topRight':
			return [parentSize[0] - HANDLE_SIZE / 2, -HANDLE_SIZE / 2];
		case 'bottomRight':
			return [parentSize[0] - HANDLE_SIZE / 2, parentSize[1] - HANDLE_SIZE / 2];
		case 'bottomLeft':
			return [-HANDLE_SIZE / 2, parentSize[1] - HANDLE_SIZE / 2];
	}
}

interface ResizeCornerHandleProps {
	position: TPosition;
	parentSize: COMP.Size;
	onPointerDown: (e: React.PointerEvent) => void;
	onPointerUp: (e: React.PointerEvent) => void;
}

type TPosition = 'topLeft' | 'topRight' | 'bottomRight' | 'bottomLeft';
