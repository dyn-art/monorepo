import React from 'react';
import type { TArgsMapType, TDateTimeModificationInput, TModificationScript } from '@dyn/arb-dtif';
import type { Artboard } from '@dyn/arb-svg-builder';
import { DateTimePicker } from '@dyn/ui';

function getDefaultDate(
	maybeUnixTimestamp: TDateTimeModificationInput['default']
): Date | undefined {
	if (maybeUnixTimestamp === 'NOW') {
		return new Date();
	} else if (typeof maybeUnixTimestamp === 'number') {
		return new Date(maybeUnixTimestamp * 1000);
	}
	return undefined;
}

export const DateTimeInput: React.FC<TProps> = (props) => {
	const { artboard, script } = props;
	const [value, setValue] = React.useState<Date | undefined>(
		getDefaultDate(script.inputVariant.default)
	);
	const [error, setError] = React.useState<string | null>(null);

	const onDateTimeUpdate = React.useCallback(
		(date?: Date) => {
			setValue(date);
			setError(null);

			if (date == null) {
				setError('Date can not be empty!');
				return;
			}

			const argsMap: TArgsMapType<TDateTimeModificationInput> = { input: date.getTime() };
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
			<DateTimePicker
				dateTime={value}
				onDateTimeUpdate={onDateTimeUpdate}
				withTime={script.inputVariant.withTime}
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
	script: TModificationScript<TDateTimeModificationInput>;
}
