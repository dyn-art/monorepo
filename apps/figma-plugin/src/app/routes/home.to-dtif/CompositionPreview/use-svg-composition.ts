import React from 'react';
import { applyCanvasDimensions, prepareDtifComposition, type COMP } from '@dyn/figma-to-dtif';
import { Composition, initWasm, SvgRenderer } from '@dyn/svg-comp';

export const useSvgComposition = (
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
	const [preparedDtif, setPreparedDtif] = React.useState<COMP.DtifComposition | null>(null);

	// Load WASM and rustify DTIF
	React.useEffect(() => {
		let isMounted = true;
		setIsLoading(true);

		const initializeAndRustify = async (): Promise<void> => {
			await initWasm();
			if (dtif && isMounted) {
				const rustified = await prepareDtifComposition(dtif);
				setPreparedDtif(rustified);
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
		if (preparedDtif != null && svgContainerRef.current != null && composition == null) {
			try {
				const newComposition = new Composition({
					dtif: applyCanvasDimensions(preparedDtif, {
						width: dimensions.width,
						height: dimensions.height
					}),
					interactive: false
				});
				newComposition.renderer = new SvgRenderer(newComposition, {
					domElement: svgContainerRef.current as Element
				});
				newComposition.update();

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
	}, [preparedDtif, svgContainerRef.current, ...deps]);

	return { svgContainerRef, composition, isLoading };
};

interface UseSVGCompositionProps {
	dtif?: COMP.DtifComposition;
	deps?: React.DependencyList;
	dimensions: TDimensions;
}

interface TDimensions {
	width: number;
	height: number;
}
