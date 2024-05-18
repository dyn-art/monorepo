import React from 'react';
import {
	hexToRgb,
	isHexColor,
	rgbaToRgb,
	rgbToHex,
	type TRgbaColor,
	type TRgbColor
} from '@dyn/utils';

import { AdvancedInput } from '../../primitive';

export const ColorText: React.FC<TProps> = (props) => {
	const { rgba, onRgbaUpdate } = props;

	const { rgb, alpha } = React.useMemo(() => rgbaToRgb(rgba), [rgba]);
	const hex = React.useMemo(() => rgbToHex(rgb), [rgb]);

	const onHexChange = React.useCallback(
		(e: React.ChangeEvent<HTMLInputElement>) => {
			const maybeHex = e.target.value;
			if (isHexColor(maybeHex)) {
				const newRgb = hexToRgb(maybeHex);
				if (newRgb != null) {
					onRgbaUpdate([newRgb[0], newRgb[1], newRgb[2], alpha]);
				}
			}
		},
		[alpha, onRgbaUpdate]
	);

	const onAlphaChange = React.useCallback(
		(e: React.ChangeEvent<HTMLInputElement>) => {
			const newAlpha = parseFloat(e.currentTarget.value);
			if (!isNaN(newAlpha)) {
				onRgbaUpdate([rgb[0], rgb[1], rgb[2], newAlpha]);
			}
		},
		[onRgbaUpdate, rgb]
	);

	const onRgbChange = React.useCallback(
		(colorIndex: 0 | 1 | 2, e: React.ChangeEvent<HTMLInputElement>) => {
			const newValue = parseInt(e.currentTarget.value, 10);
			if (!isNaN(newValue)) {
				const newRgb: TRgbColor = [...rgb];
				newRgb[colorIndex] = newValue;
				onRgbaUpdate([newRgb[0], newRgb[1], newRgb[2], alpha]);
			}
		},
		[alpha, onRgbaUpdate, rgb]
	);

	return (
		<div className="mt-2 grid grid-cols-3 gap-2">
			{/* HEX Input */}
			<div className="relative col-span-2">
				<AdvancedInput className="pl-10" id="custom" onChange={onHexChange} size="sm" value={hex}>
					<div className="pointer-events-none absolute inset-y-0 left-0 flex items-center pl-3 text-xs">
						<p className="mt-0.5 text-gray-400">HEX</p>
					</div>
				</AdvancedInput>
			</div>

			{/* Alpha Input */}
			<div className="relative col-span-1">
				<AdvancedInput
					className="no-spinner pl-6"
					id="alpha"
					onChange={onAlphaChange}
					size="sm"
					type="number"
					value={alpha}
				>
					<div className="pointer-events-none absolute inset-y-0 left-0 flex items-center pl-3 text-xs">
						<p className="mt-0.5 text-gray-400">A</p>
					</div>
				</AdvancedInput>
			</div>

			{/* RGB Inputs */}
			<div className="relative col-span-1">
				<AdvancedInput
					className="no-spinner pl-6"
					id="red"
					onChange={(e) => {
						onRgbChange(0, e);
					}}
					size="sm"
					type="number"
					value={rgb[0]}
				>
					<div className="pointer-events-none absolute inset-y-0 left-0 flex items-center pl-3 text-xs">
						<p className="mt-0.5 text-gray-400">R</p>
					</div>
				</AdvancedInput>
			</div>

			<div className="relative col-span-1">
				<AdvancedInput
					className="no-spinner pl-6"
					id="green"
					onChange={(e) => {
						onRgbChange(1, e);
					}}
					size="sm"
					type="number"
					value={rgb[1]}
				>
					<div className="pointer-events-none absolute inset-y-0 left-0 flex items-center pl-3 text-xs">
						<p className="mt-0.5 text-gray-400">G</p>
					</div>
				</AdvancedInput>
			</div>

			<div className="relative col-span-1">
				<AdvancedInput
					className="no-spinner pl-6"
					id="blue"
					onChange={(e) => {
						onRgbChange(2, e);
					}}
					size="sm"
					type="number"
					value={rgb[2]}
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
