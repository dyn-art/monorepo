'use client';

import React from 'react';
import type { Composition } from '@dyn/svg-comp';

import { useOutputEvent } from '../../hooks';

export const CanvasControl: React.FC<TProps> = (props) => {
	const { composition } = props;
	useOutputEvent(composition, 'CompositionChange');
	const selectionChangeEvent = useOutputEvent(composition, 'SelectionChange');
	const selection = React.useMemo(
		() => selectionChangeEvent?.selected ?? [],
		[selectionChangeEvent]
	);

	console.log('[CanvasControl]', { selection });

	return (
		<svg
			className="pointer-events-none absolute left-0 top-0"
			height={composition.height}
			id="canvas-control"
			overflow="visible"
			viewBox={`0 0 ${composition.width} ${composition.height}`}
			width={composition.width}
		>
			{/* TODO */}
		</svg>
	);
};

interface TProps {
	composition: Composition;
}
