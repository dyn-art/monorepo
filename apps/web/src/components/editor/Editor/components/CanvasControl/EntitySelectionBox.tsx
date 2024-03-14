import React from 'react';
import type { COMP } from '@dyn/dtif-comp';
import type { Composition } from '@dyn/svg-comp';

import { useEntity, useViewportFactor } from '../../hooks';

export const EntitySelectionBox: React.FC<TProps> = (props) => {
	const { composition, entity } = props;
	const { Size: sizeData, Transform: transformData } = useEntity(composition, entity, [
		'Size',
		'Transform'
	]);
	const factor = useViewportFactor(composition);

	if (sizeData == null || transformData == null) {
		return;
	}
	const { size } = sizeData;
	const { rotationDeg: rotation, translation } = transformData;

	return (
		<g
			style={{
				transform: `translate(${(translation[0] - composition.viewport.physicalPosition[0]) * factor[0]}px, ${
					(translation[1] - composition.viewport.physicalPosition[1]) * factor[1]
				}px) rotate(${-rotation}deg)`
			}}
		>
			{/* Selection Border */}
			<rect
				className="pointer-events-none fill-transparent stroke-blue-400 stroke-1"
				height={size[1] * factor[1]}
				width={size[0] * factor[0]}
				x={0}
				y={0}
			/>
		</g>
	);
};

interface TProps {
	entity: COMP.Entity;
	composition: Composition;
}
