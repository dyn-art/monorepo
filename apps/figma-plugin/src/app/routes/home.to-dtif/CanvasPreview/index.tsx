import { deepCopy } from '@ibg/utils';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import React from 'react';
import { ReadonlyEditor } from '@dyn/cnv-svg-editor';
import { prepareDtif, type CNV } from '@dyn/figma-to-dtif';
import { Button, ClipboardCopyIcon, FrameIcon, ScrollArea, Skeleton } from '@dyn/ui';

import { appHandler } from '../../../app-handler';
import { copyToClipboard } from '../../../core/utils';
import { useAppCallback } from '../../../hooks';

import './styles.css';

const WIDTH = 364;
const HEIGHT = 256;

const queryClient = new QueryClient();

export const CanvasPreview: React.FC<TProps> = (props) => {
	const { isTransforming } = props;

	const [dtif, setDtif] = React.useState<CNV.DtifCanvas | null>(null);
	const [preparedDtif, setPreparedDtif] = React.useState<CNV.DtifCanvas | null>(null);
	const [isPreparing, setIsPreparing] = React.useState(false);

	// =========================================================================
	// Lifecycle
	// =========================================================================

	useAppCallback(
		appHandler,
		{
			type: 'plugin.message',
			key: 'intermediate-format-export-result',
			callback: async (_, args) => {
				if (args.type === 'success') {
					setIsPreparing(true);
					setPreparedDtif(await prepareDtif(deepCopy(args.content)));
					setIsPreparing(false);

					setDtif(args.content);
					await copyToClipboard(JSON.stringify(args.content));
				}
			}
		},
		[]
	);

	// =========================================================================
	// UI
	// =========================================================================

	if (isTransforming || (dtif == null && !isPreparing)) {
		return null;
	}

	return (
		<ScrollArea className="mt-4">
			<div className="flex flex-row items-center gap-1 text-blue-400">
				<FrameIcon className="h-4 w-4" />
				<kbd className="bg-muted text-muted-foreground pointer-events-none inline-flex select-none items-center gap-1 rounded border px-1.5 font-mono text-[10px] font-medium opacity-100">
					<h4 className="text-lg">Frame</h4>
				</kbd>
			</div>
			<div
				className="preview border-base-300 mt-2 flex items-center justify-center overflow-hidden border"
				style={{ width: WIDTH, height: HEIGHT }}
			>
				<QueryClientProvider client={queryClient}>
					{preparedDtif != null && !isPreparing ? (
						<ReadonlyEditor dtif={preparedDtif} />
					) : (
						<Skeleton className="flex h-full w-full items-center justify-center rounded-none">
							Resolving Assets
						</Skeleton>
					)}
				</QueryClientProvider>
			</div>
			<div className="p-x-1 mt-2 flex flex-row justify-between">
				<Button>Open in dyn.art</Button>
				<Button
					variant={'secondary'}
					size={'icon'}
					onClick={() => {
						copyToClipboard(JSON.stringify(dtif));
					}}
				>
					<ClipboardCopyIcon className="h-4 w-4" />
				</Button>
			</div>
		</ScrollArea>
	);
};

interface TProps {
	isTransforming: boolean;
}
