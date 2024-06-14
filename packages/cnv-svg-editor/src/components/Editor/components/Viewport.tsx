import React from 'react';
import type { CNV } from '@dyn/cnv-dtif';
import type { Canvas } from '@dyn/cnv-svg-builder';
import { Button, Skeleton } from '@dyn/ui';
import { useSvgCanvas } from '@/hooks';

import { useCursorStyle } from '../hooks';
import { CanvasControl } from './CanvasControl';
import { Toolbar } from './Toolbar';

export const Viewport: React.FC<TViewportProps> = (props) => {
	const { viewportRef, dtif, onLoadedCanvas } = props;
	const svgContainerRef = React.useRef<HTMLDivElement>(null);
	const { canvas, isWasmLoading } = useSvgCanvas({ svgContainerRef, dtif });
	const cursor = useCursorStyle(canvas ?? undefined);

	React.useEffect(() => {
		if (canvas != null) {
			onLoadedCanvas?.(canvas);
		}
	}, [canvas, onLoadedCanvas]);

	return (
		<div className="bg-muted/50 relative h-full w-full" ref={viewportRef}>
			{isWasmLoading ? <Skeleton className="h-full w-full rounded-none" /> : null}
			<div ref={svgContainerRef} style={{ cursor }}>
				{canvas != null && <CanvasControl canvas={canvas} />}
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
	dtif: CNV.DtifCanvas;
	onLoadedCanvas?: (canvas: Canvas) => void;
}
