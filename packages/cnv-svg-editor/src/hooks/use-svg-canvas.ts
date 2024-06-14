import { useQuery } from '@tanstack/react-query';
import React from 'react';
import type { CNV } from '@dyn/cnv-dtif';
import { createCanvas, type Canvas } from '@dyn/cnv-svg-builder';

export function useSvgCanvas(config: TUseCanvasConfig): {
	canvas: Canvas | null;
	isWasmLoading: boolean;
} {
	const { dtif, svgContainerRef, interactive = true } = config;
	const [canvas, setCanvas] = React.useState<Canvas | null>(null);

	// https://www.youtube.com/watch?v=vxkbf5QMA2g
	const { data: isWasmLoaded, isLoading: isWasmLoading } = useQuery({
		queryKey: ['wasm'],
		queryFn: async () => {
			const { initWasm } = await import('@dyn/cnv-svg-builder');
			await initWasm();
			return true;
		},
		refetchOnWindowFocus: false
	});

	React.useEffect(() => {
		const target = svgContainerRef.current;
		if (!isWasmLoaded || target == null) {
			return;
		}

		const newCanvas = createCanvas({
			dtif,
			domElement: target as HTMLElement,
			interactive
		});
		setCanvas(newCanvas);

		return () => {
			newCanvas.unmount();
		};
	}, [isWasmLoaded, dtif, svgContainerRef]);

	return { isWasmLoading, canvas };
}

export interface TUseCanvasConfig {
	dtif: CNV.DtifCanvas;
	svgContainerRef: React.RefObject<HTMLDivElement>;
	interactive?: boolean;
}
