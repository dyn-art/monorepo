import React from 'react';
import { Composition, createSVGComposition, initWasm } from '@dyn/dtom';

import { COMPOSITION_WITH_ONE_RECT } from './test-data';

export const useSVGComposition = (props: UseSVGCompositionProps) => {
	const { width, height } = props;
	const svgContainerRef = React.useRef<HTMLDivElement>(null);
	const [composition, setComposition] = React.useState<Composition | null>(null);

	let isMounted = true; // https://github.com/facebook/react/issues/24502
	React.useEffect(() => {
		(async () => {
			if (svgContainerRef.current && composition == null && isMounted) {
				const newComposition = await createComposition({
					width,
					height,
					element: svgContainerRef.current
				});
				setComposition(newComposition);
				newComposition.update();
			}
		})();
		return () => {
			isMounted = false;
			if (composition != null) {
				composition.destory();
			}
		};
	}, [width, height, svgContainerRef.current]);

	return svgContainerRef;
};

async function createComposition(config: {
	width: number;
	height: number;
	element: Element;
}): Promise<Composition> {
	const { width, height, element } = config;
	await initWasm();

	const composition = createSVGComposition({
		width,
		height,
		renderer: {
			domElement: element
		},
		dtif: COMPOSITION_WITH_ONE_RECT(width, height)
	});

	return composition;
}

type UseSVGCompositionProps = {
	width: number;
	height: number;
};
