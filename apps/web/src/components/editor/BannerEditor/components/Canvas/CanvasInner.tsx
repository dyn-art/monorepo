'use client';

import React from 'react';
import type { Composition } from '@dyn/svg-composition';

import { useCursorStyle } from '../../hooks';
import { CanvasControl } from './CanvasControl';

export const CanvasInner: React.FC<TProps> = (props) => {
	const { composition, svgContainerRef, ...other } = props;
	const cursor = useCursorStyle(composition);

	return (
		<div {...other} style={{ ...other.style, cursor }}>
			{composition ? <CanvasControl composition={composition} /> : null}
			<div ref={svgContainerRef} />
		</div>
	);
};

type TProps = {
	composition?: Composition;
	svgContainerRef: React.RefObject<HTMLDivElement>;
} & React.HTMLAttributes<HTMLDivElement>;
