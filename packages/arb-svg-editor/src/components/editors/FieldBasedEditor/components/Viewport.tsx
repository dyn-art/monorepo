import React from 'react';
import type { ARB } from '@dyn/arb-dtif';
import type { Artboard } from '@dyn/arb-svg-builder';
import { Skeleton } from '@dyn/ui';
import { useSvgArtboard } from '@/hooks';

export const Viewport: React.FC<TViewportProps> = (props) => {
	const { viewportRef, dtif, onLoadedArtboard } = props;
	const svgContainerRef = React.useRef<HTMLDivElement>(null);
	const { artboard, isWasmLoading } = useSvgArtboard({
		svgContainerRef,
		dtif,
		interactive: false
	});

	React.useEffect(() => {
		if (artboard != null) {
			onLoadedArtboard?.(artboard);
		}
	}, [artboard, onLoadedArtboard]);

	return (
		<div className="relative h-full w-full bg-gray-100" ref={viewportRef}>
			{isWasmLoading ? <Skeleton className="h-full w-full rounded-none" /> : null}
			<div ref={svgContainerRef} />
		</div>
	);
};

export interface TViewportProps {
	viewportRef: React.RefObject<HTMLDivElement>;
	dtif?: ARB.DtifArtboard;
	onLoadedArtboard?: (artboard: Artboard) => void;
}
