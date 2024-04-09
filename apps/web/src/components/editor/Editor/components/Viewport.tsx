import React from 'react';
import type { COMP } from '@dyn/dtif-comp';
import type { Composition } from '@dyn/svg-comp';
import { Button, Skeleton } from '@dyn/ui';

import { CURSOR } from '../cursor';
import { useSvgComposition } from '../hooks';
import { CanvasControl } from './CanvasControl';
import { ToolsBar } from './ToolsBar';

export const Viewport: React.FC<TViewportProps> = (props) => {
	const { viewportRef, dtif, onLoadedComposition } = props;
	const svgContainerRef = React.useRef<HTMLDivElement>(null);
	const { composition, isWasmLoading } = useSvgComposition({ svgContainerRef, dtif });

	React.useEffect(() => {
		if (composition != null) {
			onLoadedComposition?.(composition);
		}
	}, [composition, onLoadedComposition]);

	return (
		<div
			className="relative h-full w-full bg-gray-100"
			ref={viewportRef}
			style={{ cursor: CURSOR.default() }}
		>
			{isWasmLoading ? <Skeleton className="h-full w-full" /> : null}
			<div ref={svgContainerRef} />
			{composition != null && <CanvasControl composition={composition} />}
			{composition != null && <ToolsBar composition={composition} />}
			{composition != null && (
				<Button
					className="absolute bottom-2 right-2"
					onClick={() => {
						composition.tempDevLog();
					}}
				>
					Log
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
