import React from 'react';

import { AdvancedInput } from '../../primitive';
import { Paint } from './Paint';
import type { TGradientPaint } from './types';

export const GradientPaintInputRow: React.FC<TProps> = (props) => {
	const { paint, onPopoverTriggerClick } = props;

	return (
		<div className="flex flex-row justify-start">
			<AdvancedInput
				childrenAfter={<div />}
				className="pl-8 shadow-none disabled:cursor-default disabled:opacity-100"
				disabled
				value={paint.variant.type === 'Linear' ? 'Linear Gradient' : 'Radial Gradient'}
			>
				<div className="absolute inset-y-0 left-2 flex items-center">
					<button
						className="cursor-pointer overflow-hidden rounded-sm border-[1px] border-black  hover:border-2 active:scale-105"
						onClick={onPopoverTriggerClick}
						type="button"
					>
						<Paint paint={paint} size={[16, 16]} />
					</button>
				</div>
			</AdvancedInput>
		</div>
	);
};

interface TProps {
	paint: TGradientPaint;
	onPopoverTriggerClick: React.HTMLAttributes<HTMLButtonElement>['onClick'];
}
