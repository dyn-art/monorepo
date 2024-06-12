import React from 'react';
import type { TArgsMapType, TModificationScript, TNumberModificationInput } from '@dyn/comp-dtif';
import type { Composition } from '@dyn/comp-svg-builder';
import { AdvancedInput } from '@dyn/ui';

export const NumberInput: React.FC<TProps> = (props) => {
	const { composition, script } = props;
	const [value, setValue] = React.useState<number>(script.inputVariant.default.input);
	const [error, setError] = React.useState<string | null>(null);

	const onChange = React.useCallback((e: React.ChangeEvent<HTMLInputElement>) => {
		setValue(parseFloat(e.target.value));
	}, []);

	const onFocus = React.useCallback(
		(focus: boolean) => {
			if (focus) {
				return;
			}
			setError(null);

			const argsMap: TArgsMapType<TNumberModificationInput> = { input: value };
			const scriptError = composition.executeScript({
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
				composition.update();
			}
		},
		[composition, value, script.id]
	);

	return (
		<fieldset className="w-full rounded-lg border p-4">
			<legend className="-ml-1 px-1 text-sm font-medium">{script.displayName}</legend>
			<AdvancedInput
				childrenAfter={<div />}
				defaultValue={value}
				max={script.inputVariant.max}
				min={script.inputVariant.min}
				onBlur={() => {
					onFocus(false);
				}}
				onChange={onChange}
				onFocus={() => {
					onFocus(true);
				}}
				type="number"
				variant={error != null ? 'destructive' : 'default'}
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
	composition: Composition;
	script: TModificationScript<TNumberModificationInput>;
}
