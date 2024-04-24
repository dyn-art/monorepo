import React from 'react';
import type { COMP } from '@dyn/dtif-comp';
import type { Composition } from '@dyn/svg-comp';
import { Skeleton } from '@dyn/ui';
import { useSvgComposition } from '@/hooks';

export const Viewport: React.FC<TViewportProps> = (props) => {
	const { viewportRef, dtif, onLoadedComposition } = props;
	const svgContainerRef = React.useRef<HTMLDivElement>(null);
	const { composition, isWasmLoading } = useSvgComposition({
		svgContainerRef,
		dtif,
		interactive: false
	});

	React.useEffect(() => {
		if (composition != null) {
			onLoadedComposition?.(composition);
		}
	}, [composition, onLoadedComposition]);

	return (
		<div className="relative h-full w-full bg-gray-100" ref={viewportRef}>
			{isWasmLoading ? <Skeleton className="h-full w-full rounded-none" /> : null}
			<div ref={svgContainerRef} />
		</div>
	);
};

export interface TViewportProps {
	viewportRef: React.RefObject<HTMLDivElement>;
	dtif: COMP.DtifComposition;
	onLoadedComposition?: (composition: Composition) => void;
}
