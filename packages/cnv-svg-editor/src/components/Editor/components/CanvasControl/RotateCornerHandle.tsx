import React from 'react';
import type { CNV } from '@dyn/cnv-dtif';

const INTERACTION_AREA_SIZE = 20;

export const RotateCornerHandle: React.FC<RotateHandleProps> = (props) => {
	const { position, parentSize, offset, ...other } = props;

	const [x, y] = React.useMemo<CNV.Vec2>(
		() => calculatePosition(position, parentSize, offset),
		[position, parentSize, offset]
	);

	return (
		<g id={`rotate-handle-${position}`} {...other}>
			<rect
				// className="stroke-red-500 stroke-2"
				fill="transparent"
				height={INTERACTION_AREA_SIZE}
				width={INTERACTION_AREA_SIZE}
				x={x}
				y={y}
			/>
		</g>
	);
};

function calculatePosition(position: TPosition, parentSize: CNV.Size, offset: number): CNV.Vec2 {
	switch (position) {
		case 'topLeft':
			return [-INTERACTION_AREA_SIZE / 2 - offset, -INTERACTION_AREA_SIZE / 2 - offset];
		case 'topRight':
			return [
				parentSize[0] - INTERACTION_AREA_SIZE / 2 + offset,
				-INTERACTION_AREA_SIZE / 2 - offset
			];
		case 'bottomRight':
			return [
				parentSize[0] - INTERACTION_AREA_SIZE / 2 + offset,
				parentSize[1] - INTERACTION_AREA_SIZE / 2 + offset
			];
		case 'bottomLeft':
			return [
				-INTERACTION_AREA_SIZE / 2 - offset,
				parentSize[1] - INTERACTION_AREA_SIZE / 2 + offset
			];
	}
}

interface RotateHandleProps extends React.SVGAttributes<SVGGElement> {
	position: TPosition;
	parentSize: CNV.Size;
	offset: number;
}

type TPosition = 'topLeft' | 'topRight' | 'bottomRight' | 'bottomLeft';
