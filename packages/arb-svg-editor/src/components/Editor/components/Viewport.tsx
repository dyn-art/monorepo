import React from 'react';
import type { ARB } from '@dyn/arb-dtif';
import type { Artboard } from '@dyn/arb-svg-builder';
import { Button, Skeleton } from '@dyn/ui';
import { useSvgArtboard } from '@/hooks';

import { useCursorStyle } from '../hooks';
import { ArtboardControl } from './ArtboardControl';
import { Toolbar } from './Toolbar';

export const Viewport: React.FC<TViewportProps> = (props) => {
	const { viewportRef, dtif, onLoadedArtboard } = props;
	const svgContainerRef = React.useRef<HTMLDivElement>(null);
	const { canvas, isWasmLoading } = useSvgArtboard({ svgContainerRef, dtif });
	const cursor = useCursorStyle(canvas ?? undefined);

	React.useEffect(() => {
		if (canvas != null) {
			onLoadedArtboard?.(canvas);
		}
	}, [canvas, onLoadedArtboard]);

	return (
		<div className="bg-muted/50 relative h-full w-full" ref={viewportRef}>
			{isWasmLoading ? <Skeleton className="h-full w-full rounded-none" /> : null}
			<div ref={svgContainerRef} style={{ cursor }}>
				{canvas != null && <ArtboardControl canvas={canvas} />}
			</div>
			{canvas != null && <Toolbar canvas={canvas} />}
			{canvas != null && (
				<div className="absolute bottom-2 right-2 flex flex-row justify-center gap-2">
					<Button
						onClick={() => {
							console.log(canvas.toString());
						}}
					>
						To String
					</Button>
					<Button
						onClick={() => {
							canvas.executeScript({
								id: 'test',
								argsMap: {
									x: 100,
									y: 100,
									nodeId: 'n30'
								}
							});
							canvas.update();
						}}
					>
						Run Script
					</Button>
					<Button
						onClick={() => {
							canvas.emitInputEvent('Core', { type: 'FocusRootNodes' });
							canvas.update();
						}}
					>
						Focus
					</Button>
				</div>
			)}
		</div>
	);
};

export interface TViewportProps {
	viewportRef: React.RefObject<HTMLDivElement>;
	dtif: ARB.DtifArtboard;
	onLoadedArtboard?: (canvas: Artboard) => void;
}
