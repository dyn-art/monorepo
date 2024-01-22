import React from 'react';
import type { COMP } from '@dyn/figma-to-dtif';
import { Button, ClipboardCopyIcon, FrameIcon, ScrollArea, SpinnerIcon } from '@dyn/ui';

import { appHandler } from '../../../app-handler';
import { useAppCallback } from '../../../hooks';

import './styles.css';

import { copyToClipboard } from '../../../core/utils';
import { useSVGComposition } from './use-svg-composition';

const WIDTH = 364;
const HEIGHT = 256;

export const CompositionPreview: React.FC<TProps> = (props) => {
	const { isTransforming } = props;

	const [dtif, setDTIF] = React.useState<COMP.DTIFComposition | null>(null);
	const { svgContainerRef, isLoading } = useSVGComposition({
		dtif: dtif ?? undefined,
		deps: [isTransforming],
		dimensions: {
			width: WIDTH,
			height: HEIGHT
		}
	});

	// =========================================================================
	// Lifecycle
	// =========================================================================

	useAppCallback(
		appHandler,
		{
			type: 'plugin.message',
			key: 'intermediate-format-export-result',
			callback: async (instance, args) => {
				if (args.type === 'success') {
					setDTIF(args.content);
					await copyToClipboard(JSON.stringify(args.content));
				}
			}
		},
		[]
	);

	// =========================================================================
	// UI
	// =========================================================================

	if (dtif == null || isTransforming) {
		return null;
	}

	return (
		<ScrollArea className="mt-4">
			<div className="flex flex-row items-center gap-1 text-blue-400">
				<FrameIcon className="h-4 w-4" />
				<kbd className="bg-muted text-muted-foreground pointer-events-none inline-flex select-none items-center gap-1 rounded border px-1.5 font-mono text-[10px] font-medium opacity-100">
					<h4 className="text-lg">{dtif.name}</h4>
				</kbd>
			</div>
			<div
				className="preview border-base-300 mt-2 flex items-center justify-center overflow-hidden border"
				style={{ width: WIDTH, height: HEIGHT }}
			>
				{isLoading && (
					<div className="flex flex-grow flex-col items-center justify-center">
						<SpinnerIcon className="h-4 w-4 animate-spin" />
						<p className="mt-2">Loading Preview</p>
					</div>
				)}
				<div className="pointer-events-none" ref={svgContainerRef} />
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
