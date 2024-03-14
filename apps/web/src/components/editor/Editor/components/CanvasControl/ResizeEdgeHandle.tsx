import React from 'react';
import type { COMP } from '@dyn/dtif-comp';

const INTERACTION_AREA_WIDTH = 8;

export const ResizeEdgeHandle: React.FC<TProps> = (props) => {
	const { position, length, onPointerDown, onPointerUp } = props;

	const [width, height] = React.useMemo<COMP.Vec2>(
		() => [
			position === 'top' || position === 'bottom' ? length : INTERACTION_AREA_WIDTH,
			position === 'left' || position === 'right' ? length : INTERACTION_AREA_WIDTH
		],
		[position, length]
	);
	const [x, y] = React.useMemo<COMP.Vec2>(
		() => getPositionStyle(position, length),
		[position, length]
	);

	return (
		<g id={`resize-edge-${position}`} onPointerDown={onPointerDown} onPointerUp={onPointerUp}>
			<rect
				className="stroke-yellow-500 stroke-2"
				fill="transparent"
				height={height}
				width={width}
				x={x}
				y={y}
			/>
		</g>
	);
};

function getPositionStyle(position: TPosition, length: number): COMP.Vec2 {
	switch (position) {
		case 'top':
			return [0, -INTERACTION_AREA_WIDTH / 2];
		case 'right':
			return [length - INTERACTION_AREA_WIDTH / 2, 0];
		case 'bottom':
			return [0, length - INTERACTION_AREA_WIDTH / 2];
		case 'left':
			return [-INTERACTION_AREA_WIDTH / 2, 0];
	}
}

interface TProps {
	position: TPosition;
	length: number;
	onPointerDown: (e: React.PointerEvent) => void;
	onPointerUp: (e: React.PointerEvent) => void;
}

type TPosition = 'top' | 'right' | 'bottom' | 'left';
