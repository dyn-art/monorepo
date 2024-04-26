import React from 'react';
import type { TPositionModificationInput } from '@dyn/dtif-comp';
import { Input, Label } from '@dyn/ui';

export const PositionInput: React.FC<TProps> = (props) => {
	const { inputType, displayName, onChange } = props;

	const [xValue, setXValue] = React.useState<number>(inputType.default[0]);
	const [yValue, setYValue] = React.useState<number>(inputType.default[1]);

	const handleXChange = React.useCallback(
		(e: React.ChangeEvent<HTMLInputElement>) => {
			const newX = parseFloat(e.target.value);
			setXValue(newX);
			onChange?.(newX, yValue);
		},
		[yValue, onChange]
	);

	const handleYChange = React.useCallback(
		(e: React.ChangeEvent<HTMLInputElement>) => {
			const newY = parseFloat(e.target.value);
			setYValue(newY);
			onChange?.(xValue, newY);
		},
		[xValue, onChange]
	);

	return (
		<fieldset className="grid gap-6 rounded-lg border p-4">
			<legend className="-ml-1 px-1 text-sm font-medium">{displayName}</legend>
			<div className="grid grid-cols-2 gap-4">
				<div className="grid gap-3">
					<Label htmlFor="x">X</Label>
					<Input
						defaultValue={xValue}
						id="x"
						max={inputType.max?.[0]}
						min={inputType.min?.[0]}
						onChange={handleXChange}
						type="number"
					/>
				</div>
				<div className="grid gap-3">
					<Label htmlFor="y">Y</Label>
					<Input
						defaultValue={yValue}
						id="y"
						max={inputType.max?.[1]}
						min={inputType.min?.[1]}
						onChange={handleYChange}
						type="number"
					/>
				</div>
			</div>
		</fieldset>
	);
};

interface TProps {
	inputType: TPositionModificationInput;
	displayName: string;
	onChange?: (x: number, y: number) => void;
}
