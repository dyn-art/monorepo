import React from 'react';
import { rustify, type TComposition } from '@dyn/figma-to-dtif';
import { createSVGComposition, initWasm, type Composition } from '@dyn/svg-composition';

export const useSVGComposition = (
	props: UseSVGCompositionProps
): { composition: Composition | null; svgContainerRef: React.RefObject<HTMLDivElement> } => {
	const { dtif, deps = [] } = props;
	const svgContainerRef = React.useRef<HTMLDivElement>(null);
	const [composition, setComposition] = React.useState<Composition | null>(null);

	React.useEffect(() => {
		console.log({ dtif });

		(async () => {
			if (dtif != null && svgContainerRef.current) {
				await initWasm();
				const rustifiedDTIF = await rustify(dtif);

				console.log({ rustifiedDTIF });

				try {
					const newComposition = createSVGComposition({
						width: dtif.width,
						height: dtif.height,
						renderer: {
							domElement: svgContainerRef.current
						},
						dtif: rustifiedDTIF
					});
					console.log({ newComposition });

					setComposition(newComposition);
					newComposition.update();
				} catch (error) {
					console.log({ error });
				}
			}
		})().catch(() => {
			// do nothing
		});

		return () => {
			if (composition != null) {
				composition.unmount();
			}
		};
	}, [dtif, svgContainerRef.current, ...deps]);

	return { svgContainerRef, composition };
};

interface UseSVGCompositionProps {
	dtif?: TComposition;
	deps?: React.DependencyList;
}
