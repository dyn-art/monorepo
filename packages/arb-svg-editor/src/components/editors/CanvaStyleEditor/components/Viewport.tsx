import React from 'react';
import { type ARB } from '@dyn/arb-dtif';
import { type Artboard } from '@dyn/arb-svg-builder';
import { Skeleton } from '@dyn/ui';
import { ArtboardControl } from '@/components/artboard';
import { useCursorStyle, useSvgArtboard } from '@/hooks';

export const Viewport: React.FC<TViewportProps> = (props) => {
	const { viewportRef, dtif, onLoadedArtboard } = props;
	const svgContainerRef = React.useRef<HTMLDivElement>(null);
	const { artboard, isWasmLoading } = useSvgArtboard({ svgContainerRef, dtif, onLoadedArtboard });
	const cursor = useCursorStyle(artboard ?? undefined);

	return (
		<div className="bg-muted relative h-full w-full" ref={viewportRef}>
			{isWasmLoading ? <Skeleton className="h-full w-full rounded-none" /> : null}
			<div ref={svgContainerRef} style={{ cursor }}>
				{artboard != null && <ArtboardControl artboard={artboard} />}
			</div>
		</div>
	);
};

export interface TViewportProps {
	viewportRef: React.RefObject<HTMLDivElement>;
	dtif?: ARB.DtifArtboard;
	onLoadedArtboard?: (artboard: Artboard) => void;
}
