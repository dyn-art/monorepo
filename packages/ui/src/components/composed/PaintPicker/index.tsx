'use client';

import React from 'react';
import { rgbaToRgb, rgbToHex } from '@dyn/utils';

import { Popover, PopoverContent, PopoverTrigger } from '../../layout';
import { Button, Tabs, TabsContent, TabsList, TabsTrigger } from '../../primitive';
import { Paint } from './Paint';
import { GRADIENT_COLORS as GRADIENT_PAINTS } from './presets';
import { SolidPaintTab } from './SolidPaintTab';
import type { TPaint } from './types';

export const PaintPicker: React.FC<TPaintPickerProps> = (props) => {
	const { paint, onPaintUpdate } = props;
	const [activeTab, setActiveTab] = React.useState<TPaint['type']>(paint.type);

	const paintName = React.useMemo(() => {
		switch (paint.type) {
			case 'Solid': {
				const { rgb, alpha } = rgbaToRgb(paint.color);
				return `${rgbToHex(rgb)} | ${alpha * 100}%`;
			}
			case 'Gradient':
				return paint.variant.type === 'Linear' ? 'Linear Gradient' : 'Radial Gradient';
		}
	}, [paint]);

	return (
		<Popover>
			<PopoverTrigger asChild>
				<Button className={`w-[220px] justify-start text-left font-normal `} variant="outline">
					<div className="flex w-full items-center gap-2">
						<Paint
							className="rounded !bg-cover !bg-center transition-all"
							paint={paint}
							size={[16, 16]}
						/>

						<div className="flex-1 truncate">{paintName}</div>
					</div>
				</Button>
			</PopoverTrigger>
			<PopoverContent className="w-64">
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
