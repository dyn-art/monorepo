import React from 'react';

import { useSVGComposition } from './useSVGComposition';

export const MovingRects: React.FC<TProps> = (props) => {
	const { size } = props;
	const svgContainerRef = useSVGComposition({ width: size, height: size, count: 100 });

	return <div ref={svgContainerRef} />;
};

type TProps = {
	size: number;
};
