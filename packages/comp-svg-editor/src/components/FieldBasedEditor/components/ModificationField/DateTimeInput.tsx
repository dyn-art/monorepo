import React from 'react';
import {
	applyModifications,
	type TDateTimeModificationInput,
	type TInputLuaScript
} from '@dyn/comp-dtif';
import type { Composition } from '@dyn/comp-svg-builder';
import { DateTimePicker } from '@dyn/ui';
import { getJsonFunctionExecutionEnv } from '@dyn/utils';

import { runJsonFunction } from '../run-json-function';

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
	const { composition, field } = props;
	const [value, setValue] = React.useState<Date | undefined>(
		getDefaultDate(field.inputVariant.default)
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

			// eslint-disable-next-line @typescript-eslint/no-floating-promises -- ok
			(async () => {
				const processedActions = await applyModifications(
					field,
					{
						[field.key]: date.getTime()
					},
					async (jsonFunction, args) =>
						runJsonFunction(jsonFunction, args, getJsonFunctionExecutionEnv(jsonFunction))
				);

				for (const processedAction of processedActions) {
					if (processedAction.resolved) {
						composition.emitInputEvents('Core', processedAction.events);
						composition.update();
					} else {
						setError(processedAction.notMetConditions[0]?.message ?? null);
					}
				}
			})();
		},
		[field, composition]
	);

	return (
		<fieldset className="w-full rounded-lg border p-4">
			<legend className="-ml-1 px-1 text-sm font-medium">{field.displayName}</legend>
			<DateTimePicker
				dateTime={value}
				onDateTimeUpdate={onDateTimeUpdate}
				withTime={field.inputVariant.withTime}
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
	field: TInputLuaScript<string, TDateTimeModificationInput>;
}
