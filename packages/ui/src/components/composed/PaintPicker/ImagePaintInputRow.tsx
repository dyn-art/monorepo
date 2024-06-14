import { isValidAlpha } from '@ibg/utils';
import React from 'react';
import { AdvancedInput } from '@/components/primitive';

import { Paint } from './Paint';
import type { TImagePaint } from './types';

export const ImagePaintInputRow: React.FC<TProps> = (props) => {
	const { paint, onPaintUpdate, onPopoverTriggerClick } = props;
	const [opacityValue, setOpacityValue] = React.useState<number | string>(paint.opacity * 100);
	const [isOpacityValid, setIsOpacityValid] = React.useState(true);

	const onOpacityChange = React.useCallback(
		(e: React.ChangeEvent<HTMLInputElement>) => {
			const newValue = parseFloat(e.currentTarget.value);
			const newOpacity = newValue / 100;
			setOpacityValue(newValue);
			if (isValidAlpha(newOpacity)) {
				setIsOpacityValid(true);
				onPaintUpdate({ ...paint, opacity: newOpacity });
			} else {
				setIsOpacityValid(false);
			}
		},
		[onPaintUpdate, paint]
	);

	return (
		<div className="flex flex-row justify-start">
			<AdvancedInput
				childrenAfter={<div />}
				className="pl-8 shadow-none disabled:cursor-default disabled:opacity-100"
				disabled
				value="Image"
			>
				<div className="absolute inset-y-0 left-2 flex items-center">
					<button
						className="cursor-pointer overflow-hidden rounded-sm border-[1px] border-black hover:border-2 active:scale-105"
						onClick={onPopoverTriggerClick}
						type="button"
					>
						<Paint paint={paint} size={[16, 16]} />
					</button>
				</div>
			</AdvancedInput>
			<AdvancedInput
				className="no-spinner ml-1 w-16 pl-6 shadow-none"
				onChange={onOpacityChange}
				type="number"
				value={opacityValue}
				variant={isOpacityValid ? 'default' : 'destructive'}
			>
				<div className="pointer-events-none absolute inset-y-0 left-0 flex items-center pl-3 text-sm">
					<p className="mt-0.5 font-bold text-gray-400">%</p>
				</div>
			</AdvancedInput>
		</div>
	);
};

interface TProps {
	paint: TImagePaint;
	onPaintUpdate: (paint: TImagePaint) => void;
	onPopoverTriggerClick: React.HTMLAttributes<HTMLButtonElement>['onClick'];
}
