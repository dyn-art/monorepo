'use client';

import React from 'react';
import type { ARB } from '@dyn/arb-dtif';

const INTERACTION_AREA_WIDTH = 8;

export const ResizeEdgeHandle: React.FC<TProps> = (props) => {
	const { position, parentSize, ...other } = props;

	const [width, height] = React.useMemo<ARB.Vec2>(
		() => [
			position === 'top' || position === 'bottom' ? parentSize[0] : INTERACTION_AREA_WIDTH,
			position === 'left' || position === 'right' ? parentSize[1] : INTERACTION_AREA_WIDTH
		],
		[position, parentSize]
	);
	const [x, y] = React.useMemo<ARB.Vec2>(
		() => getPositionStyle(position, parentSize),
		[position, parentSize]
	);

	return (
		<g id={`resize-edge-${position}`} {...other}>
			<rect
				// className="stroke-yellow-500 stroke-2"
				fill="transparent"
				height={height}
				width={width}
				x={x}
				y={y}
			/>
		</g>
	);
};

function getPositionStyle(position: TPosition, parentSize: ARB.Size): ARB.Vec2 {
	switch (position) {
		case 'top':
			return [0, -INTERACTION_AREA_WIDTH / 2];
		case 'right':
			return [parentSize[0] - INTERACTION_AREA_WIDTH / 2, 0];
		case 'bottom':
			return [0, parentSize[1] - INTERACTION_AREA_WIDTH / 2];
		case 'left':
			return [-INTERACTION_AREA_WIDTH / 2, 0];
	}
}

interface TProps extends React.SVGAttributes<SVGGElement> {
	position: TPosition;
	parentSize: ARB.Size;
}

type TPosition = 'top' | 'right' | 'bottom' | 'left';
