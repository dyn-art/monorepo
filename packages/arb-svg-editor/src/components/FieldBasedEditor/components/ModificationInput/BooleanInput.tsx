import React from 'react';
import type { TArgsMapType, TBooleanModificationInput, TModificationScript } from '@dyn/arb-dtif';
import type { Artboard } from '@dyn/arb-svg-builder';
import { Switch } from '@dyn/ui';

export const BooleanInput: React.FC<TProps> = (props) => {
	const { canvas, script } = props;
	const [value, setValue] = React.useState<boolean>(script.inputVariant.default.input);
	const [error, setError] = React.useState<string | null>(null);

	const onCheckedChange = React.useCallback(
		(checked: boolean) => {
			setValue(checked);
			setError(null);

			const argsMap: TArgsMapType<TBooleanModificationInput> = { input: checked };
			const scriptError = canvas.executeScript({
				id: script.id,
				argsMap: argsMap
			});
			if (scriptError != null) {
				if (scriptError.type === 'Lua') {
					setError(scriptError.message);
				} else {
					// TODO: Handle Runtime and other errors
				}
			} else {
				canvas.update();
			}
		},
		[canvas, script.id]
	);

	return (
		<fieldset className="w-full rounded-lg border p-4">
			<legend className="-ml-1 px-1 text-sm font-medium">{script.displayName}</legend>
			<Switch checked={value} onCheckedChange={onCheckedChange} />
			{error != null ? (
				<p className="mt-2 text-sm text-red-600" id="email-error">
					{error}
				</p>
			) : null}
		</fieldset>
	);
};

interface TProps {
	canvas: Artboard;
	script: TModificationScript<TBooleanModificationInput>;
}
