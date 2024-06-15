'use client';

import React from 'react';
import {
	Popover,
	PopoverContent,
	PopoverTrigger,
	Tabs,
	TabsList,
	TabsTrigger
} from '@/components/primitive';

import { GradientPaintTab } from './GradientPaintTab';
import { ImagePaintTab } from './ImagePaintTab';
import { InputRow } from './InputRow';
import { SolidPaintTab } from './SolidPaintTab';
import type { TPaint } from './types';

export * from './ColorInputGrid';
export * from './GradientPaintInputRow';
export * from './SolidPaintInputRow';
export * from './helper';
export * from './presets';
export * from './types';

export const PaintPicker: React.FC<TPaintPickerProps> = (props) => {
	const { paint, onPaintUpdate, tabs = ['Solid', 'Gradient', 'Image'] } = props;
	const [activeTab, setActiveTab] = React.useState<TPaint['type']>(paint.type);

	return (
		<Popover>
			<PopoverTrigger asChild>
				<InputRow onPaintUpdate={onPaintUpdate} paint={paint} />
			</PopoverTrigger>

			<PopoverContent align="start" className="w-64" side="bottom" sideOffset={4}>
				<Tabs
					className="w-full"
					onValueChange={(value) => {
						setActiveTab(value as TPaint['type']);
					}}
					value={activeTab}
				>
					{tabs.length > 1 ? (
						<TabsList className="mb-4 w-full">
							{tabs.map((tab) => (
								<TabsTrigger className="flex-1" key={tab} value={tab}>
									{tab}
								</TabsTrigger>
							))}
						</TabsList>
					) : null}

					<SolidPaintTab inheritOpacity onPaintUpdate={onPaintUpdate} paint={paint} />
					<GradientPaintTab onPaintUpdate={onPaintUpdate} paint={paint} />
					<ImagePaintTab onPaintUpdate={onPaintUpdate} paint={paint} />
				</Tabs>
			</PopoverContent>
		</Popover>
	);
};

export interface TPaintPickerProps {
	paint: TPaint;
	onPaintUpdate: (paint: TPaint) => void;
	tabs?: TPaint['type'][];
}
