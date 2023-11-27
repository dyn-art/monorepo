import React from 'react';
import { Composition } from '@dyn/svg-composition';

import { CanvasControl } from './components';
import { useCursorStyle } from './useCursorStyle';

export const Canvas: React.FC<TProps> = (props) => {
	const { composition, svgContainerRef } = props;
	const cursor = useCursorStyle(composition);

	return (
		<div style={{ cursor }}>
			{composition && <CanvasControl composition={composition} />}
			<div ref={svgContainerRef} />
		</div>
	);
};

type TProps = {
	composition?: Composition;
	svgContainerRef: React.RefObject<HTMLDivElement>;
};
