import React from 'react';
import type { COMP } from '@dyn/comp-dtif';
import { cn, Skeleton, useSizeCallback } from '@dyn/ui';
import { useSvgComposition } from '@/hooks';

export const ReadonlyEditor: React.FC<TReadonlyEditorProps> = (props) => {
	const { dtif, className } = props;
	const svgContainerRef = React.useRef<HTMLDivElement>(null);
	const { composition, isWasmLoading } = useSvgComposition({
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
			composition?.emitInputEvents('Core', [
				{
					type: 'UpdateCompositionSize',
					size: [size.width, size.height]
				},
				{ type: 'FocusRootNodes' }
			]);
			composition?.update();
		},
		[composition]
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
	dtif: COMP.DtifComposition;
}
