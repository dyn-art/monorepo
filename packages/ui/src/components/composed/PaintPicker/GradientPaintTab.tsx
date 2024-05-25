import React from 'react';
import { TabsContent } from '@/components/primitive';

import { Paint } from './Paint';
import { GRADIENT_PAINTS } from './presets';
import type { TPaint } from './types';

export const GradientPaintTab: React.FC<TProps> = (props) => {
	const { paint, onPaintUpdate } = props;

	return (
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
	);
};

interface TProps {
	paint: TPaint;
	onPaintUpdate: (paint: TPaint) => void;
}
