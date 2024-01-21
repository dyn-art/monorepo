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
	const { dtif, deps = [], dimensions } = props;
	const svgContainerRef = React.useRef<HTMLDivElement>(null);
	const [composition, setComposition] = React.useState<Composition | null>(null);
	const [isLoading, setIsLoading] = React.useState(false);
	const [rustifiedDTIF, setRustifiedDTIF] = React.useState<COMP.DTIFComposition | null>(null);

	// Load WASM and rustify DTIF
	React.useEffect(() => {
		let isMounted = true;
		setIsLoading(true);

		const initializeAndRustify = async (): Promise<void> => {
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
				rustifiedDTIF.viewBox = calculateViewBox(dimensions, {
					width: rustifiedDTIF.width,
					height: rustifiedDTIF.height
				});
				rustifiedDTIF.width = dimensions.width;
				rustifiedDTIF.height = dimensions.height;
				const newComposition = createSVGComposition({
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

function calculateViewBox(svgDimensions: TDimensions, rectDimensions: TDimensions): COMP.ViewBox {
	const scaleX = svgDimensions.width / rectDimensions.width;
	const scaleY = svgDimensions.height / rectDimensions.height;

	// Choose the smaller scale to ensure the rectangle fits within the SVG
	const scale = Math.min(scaleX, scaleY);

	// Calculate the new dimensions of the rectangle
	const scaledWidth = rectDimensions.width * scale;
	const scaledHeight = rectDimensions.height * scale;

	// Calculate the offset to center the rectangle
	const offsetX = (svgDimensions.width - scaledWidth) / 2;
	const offsetY = (svgDimensions.height - scaledHeight) / 2;

	return {
		minX: -offsetX / scale,
		minY: -offsetY / scale,
		width: svgDimensions.width / scale,
		height: svgDimensions.height / scale
	};
}

interface UseSVGCompositionProps {
	dtif?: COMP.DTIFComposition;
	deps?: React.DependencyList;
	dimensions: TDimensions;
}

interface TDimensions {
	width: number;
	height: number;
}
