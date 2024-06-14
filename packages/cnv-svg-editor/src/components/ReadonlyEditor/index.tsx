import React from 'react';
import type { CNV } from '@dyn/cnv-dtif';
import { cn, Skeleton, useSizeCallback } from '@dyn/ui';
import { useSvgCanvas } from '@/hooks';

export const ReadonlyEditor: React.FC<TReadonlyEditorProps> = (props) => {
	const { dtif, className } = props;
	const svgContainerRef = React.useRef<HTMLDivElement>(null);
	const { canvas, isWasmLoading } = useSvgCanvas({
		svgContainerRef,
		dtif,
		interactive: false
	});

	const viewportRef = React.useRef<HTMLDivElement>(null);
	useSizeCallback(
		viewportRef,
		// Not passing the viewport size as prop to the Canvas or in the DTIF
		// because React is kinda slow updating their states
		(size) => {
			canvas?.emitInputEvents('Core', [
				{
					type: 'UpdateCanvasSize',
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
	dtif: CNV.DtifCanvas;
}
