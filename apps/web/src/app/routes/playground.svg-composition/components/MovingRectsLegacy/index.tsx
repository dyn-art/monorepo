import React from 'react';

import { useSVGComposition } from './use-svg-composition';

export const MovingRectsLegacy: React.FC<TProps> = (props) => {
	const { size } = props;
	const svgContainerRef = useSVGComposition({ width: size, height: size, count: 100 });

	return <div ref={svgContainerRef} />;
};

type TProps = {
	size: number;
};
