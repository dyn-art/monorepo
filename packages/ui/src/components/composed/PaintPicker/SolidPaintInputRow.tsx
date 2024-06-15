'use client';

import { hexToRgb, isHexColor, isValidAlpha, rgbaToRgb, rgbToHex } from '@ibg/utils';
import React from 'react';
import { AdvancedInput } from '@/components/primitive';

import { Paint } from './Paint';
import type { TSolidPaint } from './types';

export const SolidPaintInputRow: React.FC<TProps> = (props) => {
	const { paint, onPaintUpdate, onPopoverTriggerClick } = props;
	const { rgb, alpha } = React.useMemo(() => rgbaToRgb(paint.color), [paint.color]);
	const hex = React.useMemo(() => rgbToHex(rgb), [rgb]);
	const [hexValue, setHexValue] = React.useState<string>(hex);
	const [alphaValue, setAlphaValue] = React.useState<number | string>(alpha * 100);
	const [isHexValid, setIsHexValid] = React.useState(true);
	const [isAlphaValid, setIsAlphaValid] = React.useState(true);

	React.useEffect(() => {
		setHexValue(hex);
		setIsHexValid(true);
	}, [hex]);

	React.useEffect(() => {
		setAlphaValue(alpha * 100);
		setIsAlphaValid(true);
	}, [alpha]);

	const onHexChange = React.useCallback(
		(e: React.ChangeEvent<HTMLInputElement>) => {
			const maybeHex = e.target.value;
			setHexValue(maybeHex);
			if (isHexColor(maybeHex) && maybeHex.length === 7) {
				setIsHexValid(true);
				const newRgb = hexToRgb(maybeHex);
				if (newRgb != null) {
					onPaintUpdate({ type: 'Solid', color: [newRgb[0], newRgb[1], newRgb[2], alpha] });
				}
			} else {
				setIsHexValid(false);
			}
		},
		[alpha, onPaintUpdate]
	);

	const onAlphaChange = React.useCallback(
		(e: React.ChangeEvent<HTMLInputElement>) => {
			const newValue = parseFloat(e.currentTarget.value);
			const newAlpha = newValue / 100;
			setAlphaValue(newValue);
			if (isValidAlpha(newAlpha)) {
				setIsAlphaValid(true);
				onPaintUpdate({ type: 'Solid', color: [rgb[0], rgb[1], rgb[2], newAlpha] });
			} else {
				setIsAlphaValid(false);
			}
		},
		[onPaintUpdate, rgb]
	);

	return (
		<div className="flex flex-row justify-start">
			<AdvancedInput
				childrenAfter={<div />}
				className="pl-8 shadow-none"
				onChange={onHexChange}
				value={hexValue}
				variant={isHexValid ? 'default' : 'destructive'}
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
				onChange={onAlphaChange}
				type="number"
				value={alphaValue}
				variant={isAlphaValid ? 'default' : 'destructive'}
			>
				<div className="pointer-events-none absolute inset-y-0 left-0 flex items-center pl-3 text-sm">
					<p className="mt-0.5 font-bold text-gray-400">%</p>
				</div>
			</AdvancedInput>
		</div>
	);
};

interface TProps {
	paint: TSolidPaint;
	onPaintUpdate: (paint: TSolidPaint) => void;
	onPopoverTriggerClick: React.HTMLAttributes<HTMLButtonElement>['onClick'];
}
