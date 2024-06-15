import React from 'react';
import type { ARB } from '@dyn/arb-dtif';
import { cn, Skeleton, useSizeCallback } from '@dyn/ui';
import { useSvgArtboard } from '@/hooks';

export const ReadonlyEditor: React.FC<TReadonlyEditorProps> = (props) => {
	const { dtif, className } = props;
	const svgContainerRef = React.useRef<HTMLDivElement>(null);
	const { canvas, isWasmLoading } = useSvgArtboard({
		svgContainerRef,
		dtif,
		interactive: false
	});

	const viewportRef = React.useRef<HTMLDivElement>(null);
	useSizeCallback(
		viewportRef,
		// Not passing the viewport size as prop to the Artboard or in the DTIF
		// because React is kinda slow updating their states
		(size) => {
			canvas?.emitInputEvents('Core', [
				{
					type: 'UpdateArtboardSize',
					size: [size.width, size.height]
				},
				{ type: 'FocusRootNodes' }
			]);
			canvas?.update();
		},
		[canvas]
	);

	return (
		<div className={cn('h-full w-full', className)} ref={viewportRef}>
			{isWasmLoading ? <Skeleton className="h-full w-full rounded-none" /> : null}
			<div ref={svgContainerRef} />
		</div>
	);
};

export interface TReadonlyEditorProps {
	className?: string;
	dtif: ARB.DtifArtboard;
}
