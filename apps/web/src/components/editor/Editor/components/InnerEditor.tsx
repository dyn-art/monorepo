'use client';

import React from 'react';
import type { Composition } from '@dyn/svg-comp';
import { ResizableHandle, ResizablePanel, ResizablePanelGroup, Skeleton, useSize } from '@dyn/ui';

import { type TCanvasProps } from '../../Canvas';
import { useDtifFromClipboard } from '../hooks';
import { Viewport } from './Viewport';

export const InnerEditor: React.FC<TInnerEditorProps> = (props) => {
	const { dtif: defaultDtif } = props;
	const [composition, setComposition] = React.useState<Composition | null>(null);
	const { isLoading, data: dtif } = useDtifFromClipboard(defaultDtif);
	const [viewportPanelRef, viewportSize] = useSize<HTMLDivElement>();

	return (
		<ResizablePanelGroup className="flex h-full min-h-full w-full" direction="horizontal">
			<ResizablePanel defaultSize={20}>
				<div className="flex h-full items-center justify-center p-6">
					<span className="font-semibold">Layers</span>
				</div>
			</ResizablePanel>
			<ResizableHandle withHandle />
			<ResizablePanel defaultSize={60}>
				<div className="flex h-full w-full" ref={viewportPanelRef}>
					{!isLoading && dtif != null && viewportSize != null ? (
						<Viewport
							dtif={dtif}
							height={viewportSize.height}
							onLoadedComposition={setComposition}
							width={viewportSize.width}
						/>
					) : (
						<Skeleton className="h-full w-full" />
					)}
				</div>
			</ResizablePanel>
			<ResizableHandle withHandle />
			<ResizablePanel defaultSize={20}>
				<div className="flex h-full items-center justify-center p-6">
					<span className="font-semibold">Design</span>
				</div>
			</ResizablePanel>
		</ResizablePanelGroup>
	);
};

export interface TInnerEditorProps {
	dtif: TCanvasProps['dtif'];
}
