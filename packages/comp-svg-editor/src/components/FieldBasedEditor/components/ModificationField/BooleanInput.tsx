import React from 'react';
import type { TArgsMapType, TBooleanModificationInput, TModificationScript } from '@dyn/comp-dtif';
import type { Composition } from '@dyn/comp-svg-builder';
import { Switch } from '@dyn/ui';

export const BooleanInput: React.FC<TProps> = (props) => {
	const { composition, script } = props;
	const [argsMap, setArgsMap] = React.useState<TArgsMapType<TBooleanModificationInput>>(
		script.inputVariant.default
	);
	const [error, setError] = React.useState<string | null>(null);

	const onCheckedChange = React.useCallback(
		(checked: boolean) => {
			setArgsMap({ input: checked });
			setError(null);

			// eslint-disable-next-line @typescript-eslint/no-floating-promises -- ok
			(async () => {
				composition.runScripts([
					{
						id: script.id,
						argsMap
					}
				]);
				composition.update();

				// TODO: Handle error
			})();
		},
		[composition, script.id, argsMap]
	);

	return (
		<fieldset className="w-full rounded-lg border p-4">
			<legend className="-ml-1 px-1 text-sm font-medium">{script.displayName}</legend>
			<Switch checked={argsMap.input} onCheckedChange={onCheckedChange} />
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
	script: TModificationScript<TBooleanModificationInput>;
}
