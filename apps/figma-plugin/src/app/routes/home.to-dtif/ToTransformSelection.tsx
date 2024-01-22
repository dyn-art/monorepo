import React from 'react';
import {
	Button,
	ChevronDownIcon,
	ChevronUpIcon,
	Collapsible,
	CollapsibleContent,
	CollapsibleTrigger,
	FrameIcon,
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
	Separator
} from '@dyn/ui';

import type { TOnSelectFrameEvent } from '../../../types';
import { appHandler } from '../../app-handler';
import { useAppCallback } from '../../hooks';

export const ToTransformSelection: React.FC<TProps> = (props) => {
	const { isTransforming } = props;
	const [selectedFrames, setSelectedFrames] = React.useState<
		TOnSelectFrameEvent['args']['selected']
	>([]);
	const [selectedFrameIndex, setSelectedFrameIndex] = React.useState<number | null>(null);
	const [isOpen, setIsOpen] = React.useState(true);

	// =========================================================================
	// Lifecycle
	// =========================================================================

	useAppCallback(
		appHandler,
		{
			type: 'plugin.message',
			key: 'on-select-frame',
			callback: async (instance, args) => {
				const selected = args.selected;
				if (selected.length > 0) {
					setSelectedFrames(selected);
					setSelectedFrameIndex(selected.length - 1);
					setIsOpen(true);
				} else {
					setSelectedFrames([]);
					setSelectedFrameIndex(null);
				}
			}
		},
		[]
	);

	React.useEffect(() => {
		if (isTransforming) {
			setIsOpen(false);
		}
	}, [isTransforming]);

	// =========================================================================
	// Callback
	// =========================================================================

	const onTransform = React.useCallback(() => {
		if (selectedFrameIndex == null) {
			return;
		}

		const selectedFrame = selectedFrames[selectedFrameIndex];
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
		<Collapsible
			open={isOpen}
			onOpenChange={setIsOpen}
			className="flex w-full flex-col items-center"
		>
			<CollapsibleContent className="my-4 w-full">
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
			</CollapsibleContent>

			{!isOpen && (
				<CollapsibleTrigger asChild>
					<Button variant="ghost" size="sm" disabled={isTransforming} className="my-1 h-9 w-9 p-0">
						<ChevronDownIcon className="h-4 w-4" />
						<span className="sr-only">Close</span>
					</Button>
				</CollapsibleTrigger>
			)}

			<Separator />

			{isOpen && (
				<CollapsibleTrigger asChild>
					<Button variant="ghost" size="sm" className="mt-1 h-9 w-9 p-0">
						<ChevronUpIcon className="h-4 w-4" />
						<span className="sr-only">Close</span>
					</Button>
				</CollapsibleTrigger>
			)}
		</Collapsible>
	);
};

interface TProps {
	isTransforming: boolean;
}
