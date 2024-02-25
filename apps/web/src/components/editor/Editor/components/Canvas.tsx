'use client';

import { useQuery } from '@tanstack/react-query';
import React from 'react';
import { applyCanvasDimensions, type COMP } from '@dyn/comp-dtif';
import { Composition, SvgRenderer } from '@dyn/svg-comp';
import { cn, Skeleton } from '@dyn/ui';

export const Canvas: React.FC<TCanvasProps> = (props) => {
	const { dtif, width, height, onLoadedComposition, ...other } = props;

	const svgContainerRef = React.useRef<HTMLDivElement>(null);
	const [composition, setComposition] = React.useState<Composition | null>(null);

	// =========================================================================
	// Lifecycle
	// =========================================================================

	// https://www.youtube.com/watch?v=vxkbf5QMA2g
	const { data: isWasmLoaded, isLoading: isWasmLoading } = useQuery({
		queryKey: ['wasm'],
		queryFn: async () => {
			const { initWasm } = await import('@dyn/svg-comp');
			await initWasm();
			return true;
		}
	});

	React.useEffect(() => {
		if (!isWasmLoaded && svgContainerRef.current == null) {
			return;
		}
		const newComposition = new Composition({
			dtif: applyCanvasDimensions(dtif, { width, height })
		});
		newComposition.renderer = new SvgRenderer(newComposition, {
			domElement: svgContainerRef.current as Element
		});
		newComposition.update();
		setComposition(newComposition);
		if (onLoadedComposition != null) {
			onLoadedComposition(newComposition);
		}

		// TODO: REMOVE
		newComposition.logEntityComponents(0);
		newComposition.logEntityComponents(1);

		return () => {
			newComposition.unmount();
		};
	}, [isWasmLoaded, width, height, dtif, onLoadedComposition]);

	// =========================================================================
	// UI
	// =========================================================================

	if (isWasmLoading) {
		return <Skeleton className="h-full w-full" />;
	}

	return (
		<div {...other} className={cn('bg-gray-100', other.className)}>
			<div ref={svgContainerRef} />
		</div>
	);
};

export type TCanvasProps = {
	width: number;
	height: number;
	dtif: COMP.CompDtif;
	onLoadedComposition?: (composition: Composition) => void;
} & React.HTMLAttributes<HTMLDivElement>;
