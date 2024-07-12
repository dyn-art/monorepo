import React from 'react';
import type { ARB } from '@dyn/arb-dtif';
import type { Artboard } from '@dyn/arb-svg-builder';
import { Button, Skeleton } from '@dyn/ui';
import { ArtboardControl } from '@/components/artboard';
import { useCursorStyle, useSvgArtboard } from '@/hooks';

import { Toolbar } from './Toolbar';

export const Viewport: React.FC<TViewportProps> = (props) => {
	const { viewportRef, dtif, onLoadedArtboard } = props;
	const svgContainerRef = React.useRef<HTMLDivElement>(null);
	const { artboard, isWasmLoading } = useSvgArtboard({ svgContainerRef, dtif, onLoadedArtboard });
	const cursor = useCursorStyle(artboard ?? undefined);

	return (
		<div className="bg-muted/50 relative h-full w-full" ref={viewportRef}>
			{isWasmLoading ? <Skeleton className="h-full w-full rounded-none" /> : null}
			<div ref={svgContainerRef} style={{ cursor }}>
				{artboard != null && <ArtboardControl artboard={artboard} />}
			</div>
			{artboard != null && <Toolbar artboard={artboard} />}
			{artboard != null && (
				<div className="absolute bottom-2 right-2 flex flex-row justify-center gap-2">
					<Button
						onClick={() => {
							console.log(artboard.toString());
						}}
					>
						To String
					</Button>
					<Button
						onClick={() => {
							artboard.executeScript({
								id: 'test',
								argsMap: {
									x: 100,
									y: 100,
									nodeId: 'n30'
								}
							});
							artboard.update();
						}}
					>
						Run Script
					</Button>
					<Button
						onClick={() => {
							artboard.emitInputEvent('Core', { type: 'FocusRootNodes' });
							artboard.update();
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
	dtif?: ARB.DtifArtboard;
	onLoadedArtboard?: (artboard: Artboard) => void;
}
