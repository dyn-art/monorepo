import React from 'react';
import { Composition } from '@dyn/svg-composition';

export const useSVGComposition = (props: UseSVGCompositionProps) => {
	const { width, height, createComposition } = props;
	const svgContainerRef = React.useRef<HTMLDivElement>(null);
	const [composition, setComposition] = React.useState<Composition | null>(null);

	React.useEffect(() => {
		let isSubscribed = true; // https://github.com/facebook/react/issues/24502

		(async () => {
			if (svgContainerRef.current && composition == null && isSubscribed) {
				const newComposition = await createComposition({
					width,
					height,
					element: svgContainerRef.current
				});
				if (isSubscribed) {
					setComposition(newComposition);
					newComposition.update();
				} else {
					newComposition.unmount();
				}
			}
		})();

		return () => {
			if (composition != null) {
				composition.unmount();
			}
			isSubscribed = false;
		};
	}, [width, height, svgContainerRef.current]);

	return { svgContainerRef, composition };
};

type UseSVGCompositionProps = {
	width: number;
	height: number;
	createComposition: (config: {
		width: number;
		height: number;
		element: Element;
	}) => Promise<Composition>;
};
