import React from 'react';
import { rustify, type COMP } from '@dyn/figma-to-dtif';
import { createSVGComposition, initWasm, type Composition } from '@dyn/svg-composition';

export const useSVGComposition = (
	props: UseSVGCompositionProps
): {
	composition: Composition | null;
	svgContainerRef: React.RefObject<HTMLDivElement>;
	isLoading: boolean;
} => {
	const { dtif, deps = [] } = props;
	const svgContainerRef = React.useRef<HTMLDivElement>(null);
	const [composition, setComposition] = React.useState<Composition | null>(null);
	const [isLoading, setIsLoading] = React.useState(false);
	const [rustifiedDTIF, setRustifiedDTIF] = React.useState<COMP.DTIFComposition | null>(null);

	// Load WASM and rustify DTIF
	React.useEffect(() => {
		let isMounted = true;
		setIsLoading(true);

		const initializeAndRustify = async () => {
			await initWasm();
			if (dtif && isMounted) {
				const rustified = await rustify(dtif);
				setRustifiedDTIF(rustified);
			}
		};

		initializeAndRustify()
			.catch((error) => {
				console.error('Error in initializing or rustifying:', error);
			})
			.finally(() => {
				if (isMounted) {
					setIsLoading(false);
				}
			});

		return () => {
			isMounted = false;
		};
	}, [dtif]);

	// Create SVG Composition
	React.useEffect(() => {
		if (rustifiedDTIF != null && svgContainerRef.current != null && composition == null) {
			try {
				const newComposition = createSVGComposition({
					width: rustifiedDTIF.width,
					height: rustifiedDTIF.height,
					render: {
						domElement: svgContainerRef.current
					},
					dtif: rustifiedDTIF
				});

				setComposition(newComposition);
				newComposition.update();
			} catch (error) {
				console.error('Error in creating SVG composition:', error);
			}
		}

		return () => {
			if (composition != null) {
				composition.unmount();
			}
		};
	}, [rustifiedDTIF, svgContainerRef.current, ...deps]);

	return { svgContainerRef, composition, isLoading };
};

interface UseSVGCompositionProps {
	dtif?: COMP.DTIFComposition;
	deps?: React.DependencyList;
}
