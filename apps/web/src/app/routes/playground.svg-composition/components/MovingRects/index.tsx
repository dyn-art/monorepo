import React from 'react';
import { Card, CardDescription, CardHeader, CardTitle } from '@/components/layout';
import { Button } from '@/components/primitive';

import { useSVGComposition } from './useSVGComposition';

export const MovingRects: React.FC<TProps> = (props) => {
	const { size } = props;
	const { svgContainerRef, composition, selectedEntityData, selectedEntities } = useSVGComposition({
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
			<Card className="absolute bottom-4 right-4 z-50">
				<CardHeader>
					<CardTitle>
						Selected Entity: {selectedEntities.length > 0 ? selectedEntities[0] : 'none'}
					</CardTitle>
					<CardDescription>
						x: {selectedEntityData?.x ?? '-'} <br /> y: {selectedEntityData?.y ?? '-'}
					</CardDescription>
				</CardHeader>
			</Card>
		</div>
	);
};

type TProps = {
	size: number;
};
