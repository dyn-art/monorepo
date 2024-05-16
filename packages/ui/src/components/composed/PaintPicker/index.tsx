'use client';

import React from 'react';

import { Popover, PopoverContent, PopoverTrigger } from '../../layout';
import { Button, Input, Tabs, TabsContent, TabsList, TabsTrigger } from '../../primitive';
import { mapColorToCss } from './map-color-to-css';
import { mapGradientToCss } from './map-gradient-to-css';
import { GRADIENT_COLORS as GRADIENT_PAINTS, SOLID_COLORS as SOLID_PAINTS } from './presets';
import type { TPaint, TSolidPaint } from './types';

export const PaintPicker: React.FC<TPaintPickerProps> = (props) => {
	const { paint, onPaintUpdate } = props;

	return (
		<Popover>
			<PopoverTrigger asChild>
				<Button className={`w-[220px] justify-start text-left font-normal `} variant="outline">
					<div className="flex w-full items-center gap-2">
						<div
							className="h-4 w-4 rounded !bg-cover !bg-center transition-all"
							style={{
								background:
									paint.type === 'Solid' ? mapColorToCss(paint.color) : mapGradientToCss(paint)
							}}
						/>

						<div className="flex-1 truncate">
							{paint.type === 'Solid' ? mapColorToCss(paint.color) : mapGradientToCss(paint)}
						</div>
					</div>
				</Button>
			</PopoverTrigger>
			<PopoverContent className="w-64">
				<Tabs className="w-full" defaultValue={paint.type}>
					<TabsList className="mb-4 w-full">
						<TabsTrigger className="flex-1" value="Solid">
							Solid
						</TabsTrigger>
						<TabsTrigger className="flex-1" value="Gradient">
							Gradient
						</TabsTrigger>
					</TabsList>

					<TabsContent className="mt-0 flex flex-wrap gap-1" value="Solid">
						{SOLID_PAINTS.map((solidColor) => (
							<button
								className="h-6 w-6 cursor-pointer rounded-md active:scale-105"
								key={mapColorToCss(solidColor.color)}
								onClick={() => {
									onPaintUpdate(solidColor);
								}}
								style={{ background: mapColorToCss(solidColor.color) }}
								type="button"
							/>
						))}
					</TabsContent>

					<TabsContent className="mt-0 flex flex-wrap gap-1" value="Gradient">
						<div className="mb-2 flex flex-wrap gap-1">
							{GRADIENT_PAINTS.map((gradientPaint) => (
								<button
									className="h-6 w-6 cursor-pointer rounded-md active:scale-105"
									key={mapGradientToCss(gradientPaint)}
									onClick={() => {
										onPaintUpdate(gradientPaint);
									}}
									style={{ background: mapGradientToCss(gradientPaint) }}
									type="button"
								/>
							))}
						</div>
					</TabsContent>
				</Tabs>

				{paint.type === 'Solid' && (
					<Input
						className="col-span-2 mt-4 h-8"
						id="custom"
						onChange={(e) => {
							const newColor: TSolidPaint = {
								type: 'Solid',
								color: e.currentTarget.value.split(',').map(Number) as [number, number, number]
							};
							onPaintUpdate(newColor);
						}}
						value={mapColorToCss(paint.color)}
					/>
				)}
			</PopoverContent>
		</Popover>
	);
};

export interface TPaintPickerProps {
	paint: TPaint;
	onPaintUpdate: (paint: TPaint) => void;
}
