import React from 'react';
import type { COMP } from '@dyn/dtif-comp';
import type { Composition } from '@dyn/svg-comp';
import { ResizableHandle, ResizablePanel, ResizablePanelGroup, useSizeCallback } from '@dyn/ui';

import { useDtifFromClipboard } from '../hooks';
import { DesignPanel } from './DesignPanel';
import { Viewport } from './Viewport';

export const InnerEditor: React.FC<TInnerEditorProps> = (props) => {
	const { dtif: defaultDtif } = props;
	const [composition, setComposition] = React.useState<Composition | null>(null);
	const { isLoading: isDtifLoading, data: dtif } = useDtifFromClipboard(defaultDtif);
	const viewportRef = React.useRef<HTMLDivElement>(null);

	useSizeCallback(
		viewportRef,
		// Not passing the viewport size as prop to the Canvas or in the DTIF
		// because React is kinda slow updating their states
		(size) => {
			composition?.emitInputEvents('Composition', [
				{
					type: 'CompositionResized',
					size: [size.width, size.height]
				},
				{ type: 'FocusRootNodes' }
			]);
			composition?.update();
		},
		[composition]
	);

	return (
		<ResizablePanelGroup className="flex h-full min-h-full w-full" direction="horizontal">
			<ResizablePanel defaultSize={15} maxSize={25} minSize={15}>
				<div className="flex h-full items-center justify-center p-6">
					<span className="font-semibold">Layers</span>
				</div>
			</ResizablePanel>
			<ResizableHandle />
			<ResizablePanel defaultSize={60}>
				<Viewport
					dtif={dtif ?? defaultDtif}
					isDtifLoading={isDtifLoading}
					onLoadedComposition={setComposition}
					viewportRef={viewportRef}
				/>
			</ResizablePanel>
			<ResizableHandle />
			<ResizablePanel defaultSize={15} maxSize={25} minSize={15}>
				<DesignPanel />
			</ResizablePanel>
		</ResizablePanelGroup>
	);
};

export interface TInnerEditorProps {
	dtif: COMP.DtifComposition;
}
