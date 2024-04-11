import React from 'react';
import type { COMP } from '@dyn/dtif-comp';
import type { Composition } from '@dyn/svg-comp';
import { Button, Skeleton } from '@dyn/ui';

import { useCursorStyle, useSvgComposition } from '../hooks';
import { CanvasControl } from './CanvasControl';
import { Toolbar } from './Toolbar';

export const Viewport: React.FC<TViewportProps> = (props) => {
	const { viewportRef, dtif, onLoadedComposition } = props;
	const svgContainerRef = React.useRef<HTMLDivElement>(null);
	const { composition, isWasmLoading } = useSvgComposition({ svgContainerRef, dtif });
	const cursor = useCursorStyle(composition ?? undefined);

	React.useEffect(() => {
		if (composition != null) {
			onLoadedComposition?.(composition);
		}
	}, [composition, onLoadedComposition]);

	return (
		<div className="relative h-full w-full bg-gray-100" ref={viewportRef}>
			{isWasmLoading ? <Skeleton className="h-full w-full rounded-none" /> : null}
			<div ref={svgContainerRef} style={{ cursor }} />
			{composition != null && <CanvasControl composition={composition} />}
			{composition != null && <Toolbar composition={composition} />}
			{composition != null && (
				<Button
					className="absolute bottom-2 right-2"
					onClick={() => {
						console.log(composition.toString());
					}}
				>
					To String
				</Button>
			)}
		</div>
	);
};

export interface TViewportProps {
	isDtifLoading: boolean;
	viewportRef: React.RefObject<HTMLDivElement>;
	dtif: COMP.DtifComposition;
	onLoadedComposition?: (composition: Composition) => void;
}
