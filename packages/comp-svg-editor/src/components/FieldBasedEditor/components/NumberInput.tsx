import React from 'react';
import type { TNumberModificationInput } from '@dyn/comp-dtif';
import { Input } from '@dyn/ui';

export const NumberInput: React.FC<TProps> = (props) => {
	const { inputType, displayName, onChange } = props;

	const [value, setValue] = React.useState<number>(inputType.default);

	const handleChange = React.useCallback(
		(e: React.ChangeEvent<HTMLInputElement>) => {
			const newValue = parseFloat(e.target.value);
			setValue(newValue);
			onChange?.(newValue);
		},
		[onChange]
	);

	return (
		<fieldset className="w-full rounded-lg border p-4">
			<legend className="-ml-1 px-1 text-sm font-medium">{displayName}</legend>
			<Input
				defaultValue={value}
				id="number"
				max={inputType.max}
				min={inputType.min}
				onChange={handleChange}
				type="number"
			/>
		</fieldset>
	);
};

interface TProps {
	inputType: TNumberModificationInput;
	displayName: string;
	onChange?: (rotation: number) => void;
}
