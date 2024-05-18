'use client';

import React from 'react';

import { Popover, PopoverContent, PopoverTrigger } from '../../layout';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '../../primitive';
import { GradientPaintInputRow } from './GradientPaintInputRow';
import { Paint } from './Paint';
import { GRADIENT_COLORS as GRADIENT_PAINTS } from './presets';
import { SolidPaintInputRow } from './SolidPaintInputRow';
import { SolidPaintTab } from './SolidPaintTab';
import type { TPaint } from './types';

export const PaintPicker: React.FC<TPaintPickerProps> = (props) => {
	const { paint, onPaintUpdate } = props;
	const [activeTab, setActiveTab] = React.useState<TPaint['type']>(paint.type);

	return (
		<Popover>
			<PopoverTrigger asChild>
				{paint.type === 'Solid' ? (
					<SolidPaintInputRow onPaintUpdate={onPaintUpdate} paint={paint} />
				) : (
					<GradientPaintInputRow paint={paint} />
				)}
			</PopoverTrigger>

			<PopoverContent align="start" alignOffset={-8} className="w-64" side="bottom" sideOffset={12}>
				<Tabs
					className="w-full"
					onValueChange={(value) => {
						setActiveTab(value as TPaint['type']);
					}}
					value={activeTab}
				>
					<TabsList className="mb-4 w-full">
						<TabsTrigger className="flex-1" value="Solid">
							Solid
						</TabsTrigger>
						<TabsTrigger className="flex-1" value="Gradient">
							Gradient
						</TabsTrigger>
					</TabsList>

					<SolidPaintTab onPaintUpdate={onPaintUpdate} paint={paint} />

					<TabsContent className="mt-0 flex flex-wrap gap-1" value="Gradient">
						<div className="flex flex-wrap gap-1">
							{GRADIENT_PAINTS.map((gradientPaint) => (
								<button
									className="cursor-pointer overflow-hidden rounded-md active:scale-105"
									key={gradientPaint.stops.map((stop) => stop.color).join('-')}
									onClick={() => {
										onPaintUpdate(gradientPaint);
									}}
									type="button"
								>
									<Paint paint={gradientPaint} size={[24, 24]} />
								</button>
							))}
						</div>
					</TabsContent>
				</Tabs>
			</PopoverContent>
		</Popover>
	);
};

export interface TPaintPickerProps {
	paint: TPaint;
	onPaintUpdate: (paint: TPaint) => void;
}
