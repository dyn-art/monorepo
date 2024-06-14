import React from 'react';
import type { CNV } from '@dyn/cnv-dtif';

const HANDLE_SIZE = 8;
const INTERACTION_AREA_SIZE = 20;
const interactionAreaOffset = (INTERACTION_AREA_SIZE - HANDLE_SIZE) / 2;

export const ResizeCornerHandle: React.FC<ResizeCornerHandleProps> = (props) => {
	const { position, parentSize, ...other } = props;

	const [x, y] = React.useMemo<CNV.Vec2>(
		() => calculatePosition(position, parentSize),
		[position, parentSize]
	);

	return (
		<g id={`resize-corner-${position}`} {...other}>
			<rect
				className="fill-white stroke-blue-400 stroke-1"
				height={HANDLE_SIZE}
				width={HANDLE_SIZE}
				x={x}
				y={y}
			/>
			<rect
				// className="stroke-green-500 stroke-2"
				fill="transparent"
				height={INTERACTION_AREA_SIZE}
				width={INTERACTION_AREA_SIZE}
				x={x - interactionAreaOffset}
				y={y - interactionAreaOffset}
			/>
		</g>
	);
};

function calculatePosition(position: TPosition, parentSize: CNV.Size): CNV.Vec2 {
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

interface ResizeCornerHandleProps extends React.SVGAttributes<SVGGElement> {
	position: TPosition;
	parentSize: CNV.Size;
}

type TPosition = 'topLeft' | 'topRight' | 'bottomRight' | 'bottomLeft';
