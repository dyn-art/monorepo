'use client';

import React from 'react';
import type { Composition } from '@dyn/svg-comp';

import { useOutputEvent } from '../../hooks';
import { EntitySelectionBox } from './EntitySelectionBox';

export const CanvasControl: React.FC<TProps> = (props) => {
	const { composition } = props;
	useOutputEvent(composition, 'CompositionChange');
	const { selected: selectedEntities = [] } = useOutputEvent(composition, 'SelectionChange') ?? {};

	return (
		<svg
			className="pointer-events-none absolute left-0 top-0"
			height={composition.size[1]}
			id="canvas-control"
			overflow="visible"
			viewBox={`0 0 ${composition.size[0]} ${composition.size[1]}`}
			width={composition.size[0]}
		>
			<defs>
				<clipPath id="canvas-clip">
					<rect height={composition.size[1]} width={composition.size[0]} x={0} y={0} />
				</clipPath>
			</defs>
			<g className="pointer-events-auto" clipPath="url(#canvas-clip)">
				<g id="entity-selection">
					{selectedEntities.map((selectedEntity) => (
						<EntitySelectionBox
							composition={composition}
							entity={selectedEntity}
							key={selectedEntity}
						/>
					))}
				</g>
			</g>
		</svg>
	);
};

interface TProps {
	composition: Composition;
}
