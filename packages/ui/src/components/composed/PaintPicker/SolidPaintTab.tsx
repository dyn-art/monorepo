import React from 'react';
import { TabsContent } from '@/components/primitive';

import { ColorInputGrid } from './ColorInputGrid';
import { Paint } from './Paint';
import { SOLID_PAINTS } from './presets';
import type { TPaint } from './types';

export const SolidPaintTab: React.FC<TProps> = (props) => {
	const { paint, onPaintUpdate } = props;

	return (
		<TabsContent className="mt-0 flex flex-wrap gap-1" value="Solid">
			<div className="flex flex-wrap gap-1">
				{SOLID_PAINTS.map((solidPaint) => (
					<button
						className="cursor-pointer overflow-hidden rounded-md active:scale-105"
						key={solidPaint.color.join('-')}
						onClick={() => {
							onPaintUpdate(solidPaint);
						}}
						type="button"
					>
						<Paint paint={solidPaint} size={[24, 24]} />
					</button>
				))}
			</div>
			{paint.type === 'Solid' && (
				<ColorInputGrid
					onRgbaUpdate={(rgba) => {
						onPaintUpdate({ type: 'Solid', color: rgba });
					}}
					rgba={paint.color}
				/>
			)}
		</TabsContent>
	);
};

interface TProps {
	paint: TPaint;
	onPaintUpdate: (paint: TPaint) => void;
}
