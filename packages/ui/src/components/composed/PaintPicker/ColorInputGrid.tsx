import React from 'react';
import {
	hexToRgb,
	isHexColor,
	isValidAlpha,
	isValidU8,
	rgbaToRgb,
	rgbToHex,
	type TRgbaColor,
	type TRgbColor
} from '@dyn/utils';

import { AdvancedInput } from '../../primitive';

export const ColorInputGrid: React.FC<TProps> = ({ rgba, onRgbaUpdate }) => {
	const { rgb, alpha } = React.useMemo(() => rgbaToRgb(rgba), [rgba]);
	const hex = React.useMemo(() => rgbToHex(rgb), [rgb]);
	const [hexValue, setHexValue] = React.useState<string>(hex);
	const [rValue, setRValue] = React.useState<number | string>(rgb[0]);
	const [gValue, setGValue] = React.useState<number | string>(rgb[1]);
	const [bValue, setBValue] = React.useState<number | string>(rgb[2]);
	const [alphaValue, setAlphaValue] = React.useState<number | string>(alpha * 100);

	const [isHexValid, setIsHexValid] = React.useState(true);
	const [isRValid, setIsRValid] = React.useState(true);
	const [isGValid, setIsGValid] = React.useState(true);
	const [isBValid, setIsBValid] = React.useState(true);
	const [isAlphaValid, setIsAlphaValid] = React.useState(true);

	React.useEffect(() => {
		setHexValue(hex);
		setIsHexValid(true);
	}, [hex]);

	React.useEffect(() => {
		setRValue(rgb[0]);
		setGValue(rgb[1]);
		setBValue(rgb[2]);
		setAlphaValue(alpha * 100);

		setIsRValid(true);
		setIsGValid(true);
		setIsBValid(true);
		setIsAlphaValid(true);
	}, [rgb, alpha]);

	const onHexChange = React.useCallback(
		(e: React.ChangeEvent<HTMLInputElement>) => {
			const maybeHex = e.target.value;
			setHexValue(maybeHex);
			if (isHexColor(maybeHex) && maybeHex.length === 7) {
				setIsHexValid(true);
				const newRgb = hexToRgb(maybeHex);
				if (newRgb != null) {
					onRgbaUpdate([newRgb[0], newRgb[1], newRgb[2], alpha]);
				}
			} else {
				setIsHexValid(false);
			}
		},
		[alpha, onRgbaUpdate]
	);

	const onAlphaChange = React.useCallback(
		(e: React.ChangeEvent<HTMLInputElement>) => {
			const newValue = parseFloat(e.currentTarget.value);
			const newAlpha = newValue / 100;
			setAlphaValue(newValue);
			if (isValidAlpha(newAlpha)) {
				setIsAlphaValid(true);
				onRgbaUpdate([rgb[0], rgb[1], rgb[2], newAlpha]);
			} else {
				setIsAlphaValid(false);
			}
		},
		[onRgbaUpdate, rgb]
	);

	const onRgbChange = React.useCallback(
		(colorIndex: 0 | 1 | 2, e: React.ChangeEvent<HTMLInputElement>) => {
			const newValue = e.currentTarget.value;
			const numericValue = parseInt(newValue, 10);

			if (colorIndex === 0) setRValue(newValue);
			if (colorIndex === 1) setGValue(newValue);
			if (colorIndex === 2) setBValue(newValue);

			if (isValidU8(numericValue)) {
				if (colorIndex === 0) setIsRValid(true);
				if (colorIndex === 1) setIsGValid(true);
				if (colorIndex === 2) setIsBValid(true);

				const newRgb: TRgbColor = [...rgb];
				newRgb[colorIndex] = numericValue;
				onRgbaUpdate([newRgb[0], newRgb[1], newRgb[2], alpha]);
			} else {
				if (colorIndex === 0) setIsRValid(false);
				if (colorIndex === 1) setIsGValid(false);
				if (colorIndex === 2) setIsBValid(false);
			}
		},
		[alpha, onRgbaUpdate, rgb]
	);

	return (
		<div className="mt-2 grid grid-cols-3 gap-2">
			{/* HEX Input */}
			<div className="relative col-span-2">
				<AdvancedInput
					childrenAfter={<div />}
					className="pl-10"
					id="custom"
					onChange={onHexChange}
					size="sm"
					value={hexValue}
					variant={isHexValid ? 'default' : 'destructive'}
				>
					<div className="pointer-events-none absolute inset-y-0 left-0 flex items-center pl-3 text-xs">
						<p className="mt-0.5 text-gray-400">HEX</p>
					</div>
				</AdvancedInput>
			</div>

			{/* Alpha Input */}
			<div className="relative col-span-1">
				<AdvancedInput
					childrenAfter={<div />}
					className="no-spinner pl-6"
					id="alpha"
					onChange={onAlphaChange}
					size="sm"
					type="number"
					value={alphaValue}
					variant={isAlphaValid ? 'default' : 'destructive'}
				>
					<div className="pointer-events-none absolute inset-y-0 left-0 flex items-center pl-3 text-xs">
						<p className="mt-0.5 text-gray-400">A</p>
					</div>
				</AdvancedInput>
			</div>

			{/* RGB Inputs */}
			<div className="relative col-span-1">
				<AdvancedInput
					childrenAfter={<div />}
					className="no-spinner pl-6"
					id="red"
					onChange={(e) => {
						onRgbChange(0, e);
					}}
					size="sm"
					type="number"
					value={rValue}
					variant={isRValid ? 'default' : 'destructive'}
				>
					<div className="pointer-events-none absolute inset-y-0 left-0 flex items-center pl-3 text-xs">
						<p className="mt-0.5 text-gray-400">R</p>
					</div>
				</AdvancedInput>
			</div>

			<div className="relative col-span-1">
				<AdvancedInput
					childrenAfter={<div />}
					className="no-spinner pl-6"
					id="green"
					onChange={(e) => {
						onRgbChange(1, e);
					}}
					size="sm"
					type="number"
					value={gValue}
					variant={isGValid ? 'default' : 'destructive'}
				>
					<div className="pointer-events-none absolute inset-y-0 left-0 flex items-center pl-3 text-xs">
						<p className="mt-0.5 text-gray-400">G</p>
					</div>
				</AdvancedInput>
			</div>

			<div className="relative col-span-1">
				<AdvancedInput
					childrenAfter={<div />}
					className="no-spinner pl-6"
					id="blue"
					onChange={(e) => {
						onRgbChange(2, e);
					}}
					size="sm"
					type="number"
					value={bValue}
					variant={isBValid ? 'default' : 'destructive'}
				>
					<div className="pointer-events-none absolute inset-y-0 left-0 flex items-center pl-3 text-xs">
						<p className="mt-0.5 text-gray-400">B</p>
					</div>
				</AdvancedInput>
			</div>
		</div>
	);
};

interface TProps {
	rgba: TRgbaColor;
	onRgbaUpdate: (rgba: TRgbaColor) => void;
}
