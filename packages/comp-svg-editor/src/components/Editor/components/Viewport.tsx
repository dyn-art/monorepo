import React from 'react';
import type { COMP } from '@dyn/comp-dtif';
import type { Composition } from '@dyn/comp-svg-builder';
import { Button, Skeleton } from '@dyn/ui';
import { useSvgComposition } from '@/hooks';

import { useCursorStyle } from '../hooks';
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
		<div className="bg-muted/50 relative h-full w-full" ref={viewportRef}>
			{isWasmLoading ? <Skeleton className="h-full w-full rounded-none" /> : null}
			<div ref={svgContainerRef} style={{ cursor }}>
				{composition != null && <CanvasControl composition={composition} />}
			</div>
			{composition != null && <Toolbar composition={composition} />}
			{composition != null && (
				<div className="absolute bottom-2 right-2 flex flex-row justify-center gap-2">
					<Button
						onClick={() => {
							console.log(composition.toString());
						}}
					>
						To String
					</Button>
					<Button
						onClick={() => {
							composition.runScripts([
								{
									key: 'test',
									argsMap: {
										x: { type: 'Number', value: 200 },
										y: { type: 'Number', value: 200 },
										nodeId: { type: 'String', value: 'n30' }
									}
								}
							]);
							composition.update();
						}}
					>
						Run Script
					</Button>
					<Button
						onClick={() => {
							composition.emitInputEvent('Core', { type: 'FocusRootNodes' });
							composition.update();
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
	dtif: COMP.DtifComposition;
	onLoadedComposition?: (composition: Composition) => void;
}
