import React from 'react';
import type { TArgsMapType, TModificationScript, TTextModificationInput } from '@dyn/arb-dtif';
import type { Artboard } from '@dyn/arb-svg-builder';
import { AdvancedInput, AdvancedTextarea } from '@dyn/ui';

export const TextInput: React.FC<TProps> = (props) => {
	const { artboard, script } = props;
	const [value, setValue] = React.useState<string>(script.inputVariant.default.input);
	const [error, setError] = React.useState<string | null>(null);

	const onChange = React.useCallback(
		(e: React.ChangeEvent<HTMLInputElement> | React.ChangeEvent<HTMLTextAreaElement>) => {
			setValue(e.target.value);
		},
		[]
	);

	const onFocus = React.useCallback(
		(focus: boolean) => {
			if (focus) {
				return;
			}
			setError(null);

			const argsMap: TArgsMapType<TTextModificationInput> = { input: value };
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
		[artboard, value, script.id]
	);

	return (
		<fieldset className="w-full rounded-lg border p-4">
			<legend className="-ml-1 px-1 text-sm font-medium">{script.displayName}</legend>
			{script.inputVariant.area ? (
				<AdvancedTextarea
					childrenAfter={<div />}
					defaultValue={value}
					onBlur={() => {
						onFocus(false);
					}}
					onChange={onChange}
					onFocus={() => {
						onFocus(true);
					}}
					variant={error != null ? 'destructive' : 'default'}
				/>
			) : (
				<AdvancedInput
					childrenAfter={<div />}
					defaultValue={value}
					onBlur={() => {
						onFocus(false);
					}}
					onChange={onChange}
					onFocus={() => {
						onFocus(true);
					}}
					variant={error != null ? 'destructive' : 'default'}
				/>
			)}
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
	script: TModificationScript<TTextModificationInput>;
}
