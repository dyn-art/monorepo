import { useQuery } from '@tanstack/react-query';
import React from 'react';
import type { COMP } from '@dyn/dtif-comp';
import { createSvgComposition, type Composition } from '@dyn/svg-comp';

export function useSvgComposition(config: TUseCanvasConfig): {
	composition: Composition | null;
	isWasmLoading: boolean;
} {
	const { dtif, svgContainerRef } = config;
	const [composition, setComposition] = React.useState<Composition | null>(null);

	// https://www.youtube.com/watch?v=vxkbf5QMA2g
	const { data: isWasmLoaded, isLoading: isWasmLoading } = useQuery({
		queryKey: ['wasm'],
		queryFn: async () => {
			const { initWasm } = await import('@dyn/svg-comp');
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

		const newComposition = createSvgComposition({
			dtif,
			renderer: {
				domElement: target as HTMLElement
			},
			interactive: true
		});
		setComposition(newComposition);

		return () => {
			newComposition.unmount();
		};
	}, [isWasmLoaded, dtif, svgContainerRef]);

	return { isWasmLoading, composition };
}

export interface TUseCanvasConfig {
	dtif: COMP.DtifComposition;
	svgContainerRef: React.RefObject<HTMLDivElement>;
}
