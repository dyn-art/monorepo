import React from 'react';
import { Button } from '@/components/primitive';

import { useSVGComposition } from './useSVGComposition';

export const MovingRects: React.FC<TProps> = (props) => {
	const { size } = props;
	const { svgContainerRef, composition } = useSVGComposition({
		width: size,
		height: size,
		count: 100
	});

	const handleToString = React.useCallback(async () => {
		if (composition != null) {
			console.log('SVG String: ', composition?.toString());
		}
	}, [composition]);

	return (
		<div className="relative h-full w-full">
			<div ref={svgContainerRef} />
			<div className="absolute left-4 top-4 z-50 flex flex-row gap-x-2">
				<Button onClick={handleToString}>To String</Button>
			</div>
		</div>
	);
};

type TProps = {
	size: number;
};
