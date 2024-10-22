import React from 'react';
import type { TArgsMapType, TModificationScript, TRangeModificationInput } from '@dyn/arb-dtif';
import type { Artboard } from '@dyn/arb-svg-builder';
import { Slider } from '@dyn/ui';

export const RangeInput: React.FC<TProps> = (props) => {
	const { artboard, script } = props;
	const [value, setValue] = React.useState<number[]>([script.inputVariant.default.input]);
	const [error, setError] = React.useState<string | null>(null);

	const onValueChange = React.useCallback(
		(newValue: number[]) => {
			setValue(newValue);
			setError(null);

			const argsMap: TArgsMapType<TRangeModificationInput> = { input: newValue[0] ?? 0 };
			const scriptError = artboard.executeScript({
				id: script.id,
				argsMap
			});
			if (scriptError != null) {
				if (scriptError.type === 'Lua') {
					setError(scriptError.message);
				} else {
					// TODO: Handle Runtime and other errors
				}
			} else {
				artboard.update();
			}
		},
		[artboard, script.id]
	);

	return (
		<fieldset className="w-full rounded-lg border p-4">
			<legend className="-ml-1 px-1 text-sm font-medium">{script.displayName}</legend>
			<Slider
				max={script.inputVariant.max}
				min={script.inputVariant.min}
				onValueChange={onValueChange}
				step={script.inputVariant.step ?? 1}
				value={value}
			/>
			{error != null ? (
				<p className="mt-2 text-sm text-red-600" id="email-error">
					{error}
				</p>
			) : null}
		</fieldset>
	);
};

interface TProps {
	artboard: Artboard;
	script: TModificationScript<TRangeModificationInput>;
}
