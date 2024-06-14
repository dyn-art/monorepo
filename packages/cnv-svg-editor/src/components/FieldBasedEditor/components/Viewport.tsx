import React from 'react';
import type { CNV } from '@dyn/cnv-dtif';
import type { Canvas } from '@dyn/cnv-svg-builder';
import { Skeleton } from '@dyn/ui';
import { useSvgCanvas } from '@/hooks';

export const Viewport: React.FC<TViewportProps> = (props) => {
	const { viewportRef, dtif, onLoadedCanvas } = props;
	const svgContainerRef = React.useRef<HTMLDivElement>(null);
	const { canvas, isWasmLoading } = useSvgCanvas({
		svgContainerRef,
		dtif,
		interactive: false
	});

	React.useEffect(() => {
		if (canvas != null) {
			onLoadedCanvas?.(canvas);
		}
	}, [canvas, onLoadedCanvas]);

	return (
		<div className="relative h-full w-full bg-gray-100" ref={viewportRef}>
			{isWasmLoading ? <Skeleton className="h-full w-full rounded-none" /> : null}
			<div ref={svgContainerRef} />
		</div>
	);
};

export interface TViewportProps {
	viewportRef: React.RefObject<HTMLDivElement>;
	dtif: CNV.DtifCanvas;
	onLoadedCanvas?: (canvas: Canvas) => void;
}
