import React from 'react';

import { useSVGComposition } from './useSVGComposition';

export const Static: React.FC<TProps> = (props) => {
	const { size } = props;
	const svgContainerRef = useSVGComposition({ width: size, height: size });

	return <div ref={svgContainerRef} />;
};

type TProps = {
	size: number;
};
