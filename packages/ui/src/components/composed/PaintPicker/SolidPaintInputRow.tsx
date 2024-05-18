import React from 'react';
import { hexToRgb, isHexColor, isValidAlpha, rgbaToRgb, rgbToHex } from '@dyn/utils';

import { PopoverTrigger } from '../../layout';
import { AdvancedInput } from '../../primitive';
import { Paint } from './Paint';
import type { TSolidPaint } from './types';

export const SolidPaintInputRow: React.FC<TProps> = (props) => {
	const { paint, onPaintUpdate } = props;
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
			if (isHexColor(maybeHex)) {
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
				className="w-full rounded-none pl-8 shadow-none"
				onChange={onHexChange}
				value={hexValue}
				variant={isHexValid ? 'default' : 'destructive'}
			>
				<PopoverTrigger asChild>
					<button
						className="absolute inset-y-0 left-2 cursor-pointer overflow-hidden rounded-md active:scale-105"
						type="button"
					>
						<Paint paint={paint} size={[16, 16]} />
					</button>
				</PopoverTrigger>
			</AdvancedInput>
			<AdvancedInput
				className="no-spinner ml-0.5 w-16 rounded-none pl-6 shadow-none"
				onChange={onAlphaChange}
				type="number"
				value={alphaValue}
				variant={isAlphaValid ? 'default' : 'destructive'}
			>
				<div className="pointer-events-none absolute inset-y-0 left-2 flex items-center text-sm">
					<p className="mt-0.5 font-bold text-gray-400">%</p>
				</div>
			</AdvancedInput>
		</div>
	);
};

interface TProps {
	paint: TSolidPaint;
	onPaintUpdate: (paint: TSolidPaint) => void;
}
