'use client';

import React from 'react';
import type { CNV } from '@dyn/cnv-dtif';
import type { Canvas } from '@dyn/cnv-svg-builder';
import {
	ResizableHandle,
	ResizablePanel,
	ResizablePanelGroup,
	Skeleton,
	useSizeCallback
} from '@dyn/ui';
import { usePreparedDtif } from '@/hooks';

import { DesignPanel, Viewport } from './components';

export const Editor: React.FC<TEditorProps> = (props) => {
	const { dtif } = props;
	const [canvas, setCanvas] = React.useState<Canvas | null>(null);
	const viewportRef = React.useRef<HTMLDivElement>(null);
	const { data: preparedDtif, isLoading: isPreparingDtif } = usePreparedDtif(dtif);

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
		<ResizablePanelGroup className="flex h-full min-h-full w-full" direction="horizontal">
			<ResizablePanel defaultSize={15} maxSize={25} minSize={15}>
				<div className="flex h-full items-center justify-center p-6">
					<span className="font-semibold">Layers</span>
				</div>
			</ResizablePanel>
			<ResizableHandle />
			<ResizablePanel defaultSize={70}>
				{isPreparingDtif || preparedDtif == null ? (
					<Skeleton className="h-full w-full rounded-none" />
				) : (
					<Viewport dtif={preparedDtif} onLoadedCanvas={setCanvas} viewportRef={viewportRef} />
				)}
			</ResizablePanel>
			<ResizableHandle />
			<ResizablePanel defaultSize={15} maxSize={25} minSize={15}>
				<DesignPanel />
			</ResizablePanel>
		</ResizablePanelGroup>
	);
};

export interface TEditorProps {
	dtif?: CNV.DtifCanvas;
}
