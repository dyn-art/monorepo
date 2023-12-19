import React from 'react';
import {
	Button,
	FrameIcon,
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
	Separator
} from '@dyn/ui';

import type { TOnSelectFrameEvent } from '../../types';
import { appHandler } from '../app-handler';
import { useAppCallback } from '../hooks';

const ToDTIFPlugin: React.FC = () => {
	const [selectedFrames, setSelectedFrames] = React.useState<
		TOnSelectFrameEvent['args']['selected']
	>([]);
	const [selectedFrameIndex, setSelectedFrameIndex] = React.useState<number | null>(null);

	// =========================================================================
	// Lifecycle
	// =========================================================================

	useAppCallback(appHandler, {
		type: 'plugin.message',
		key: 'on-select-frame',
		callback: async (instance, args) => {
			const selected = args.selected;
			if (selected.length > 0) {
				setSelectedFrames(selected);
				setSelectedFrameIndex(selected.length - 1);
			} else {
				setSelectedFrames([]);
				setSelectedFrameIndex(null);
			}
		}
	});

	// =========================================================================
	// Callback
	// =========================================================================

	const onTransform = React.useCallback(() => {
		if (selectedFrameIndex == null) {
			return;
		}

		const selectedFrame = selectedFrames[selectedFrameIndex];
		console.log('onTransform', selectedFrame);
		if (selectedFrame != null) {
			appHandler.post('intermediate-format-export', {
				selectedElements: [{ id: selectedFrame.id, name: selectedFrame.name }]
			});
		}
	}, [selectedFrames, selectedFrameIndex]);

	// =========================================================================
	// UI
	// =========================================================================
	return (
		<>
			<div className="flex h-24 w-full flex-col items-center justify-center rounded-md border text-center text-blue-400">
				<FrameIcon className="mb-1 h-4 w-4 " />
				<p>
					Select Frame to transform <br /> in Figma canvas{' '}
					<kbd className="bg-muted text-muted-foreground pointer-events-none inline-flex h-5 select-none items-center gap-1 rounded border px-1.5 font-mono text-[10px] font-medium opacity-100">
						<span className="text-xs">⌘A</span>
					</kbd>
				</p>
			</div>
			<div className="mt-2 flex items-center justify-between">
				<Select
					defaultValue={selectedFrameIndex?.toString()}
					value={selectedFrameIndex?.toString()}
					onValueChange={(value) => {
						setSelectedFrameIndex(Number(value));
					}}
				>
					<SelectTrigger id="frame">
						<SelectValue placeholder="None selected">
							{selectedFrames[Number(selectedFrameIndex)]?.name}
						</SelectValue>
					</SelectTrigger>
					<SelectContent position="popper">
						{selectedFrames.map((plugin, index) => (
							<SelectItem key={plugin.id} value={index.toString()}>
								{plugin.name}
							</SelectItem>
						))}
					</SelectContent>
				</Select>
				<Button className="ml-2" disabled={selectedFrameIndex == null} onClick={onTransform}>
					Transform
				</Button>
			</div>
			<Separator className="my-4" />
		</>
	);
};

export default ToDTIFPlugin;
