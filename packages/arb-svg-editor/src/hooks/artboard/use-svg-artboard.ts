import { useQuery } from '@tanstack/react-query';
import React from 'react';
import type { ARB } from '@dyn/arb-dtif';
import { createArtboard, type Artboard } from '@dyn/arb-svg-builder';

export function useSvgArtboard(config: TUseArtboardConfig): {
	artboard: Artboard | null;
	isWasmLoading: boolean;
	isDtifLoading: boolean;
} {
	const { dtif, svgContainerRef, interactive = true, onLoadedArtboard } = config;
	const [artboard, setArtboard] = React.useState<Artboard | null>(null);

	// https://www.youtube.com/watch?v=vxkbf5QMA2g
	const { data: isWasmLoaded, isLoading: isWasmLoading } = useQuery({
		queryKey: ['wasm'],
		queryFn: async () => {
			const { initWasm } = await import('@dyn/arb-svg-builder');
			await initWasm();
			return true;
		},
		refetchOnWindowFocus: false
	});

	React.useEffect(() => {
		const target = svgContainerRef.current;
		if (!isWasmLoaded || target == null || dtif == null) {
			return;
		}

		const newArtboard = createArtboard({
			dtif,
			domElement: target as HTMLElement,
			interactive
		});
		setArtboard(newArtboard);
		onLoadedArtboard?.(newArtboard);

		return () => {
			newArtboard.unmount();
		};
	}, [isWasmLoaded, dtif, svgContainerRef, interactive, onLoadedArtboard]);

	return { isWasmLoading, artboard, isDtifLoading: dtif == null };
}

export interface TUseArtboardConfig {
	dtif?: ARB.DtifArtboard;
	svgContainerRef: React.RefObject<HTMLDivElement>;
	interactive?: boolean;
	onLoadedArtboard?: (artboard: Artboard) => void;
}
