import React from 'react';
import {
	applyModifications,
	type TModificationField,
	type TNumberModificationInput
} from '@dyn/comp-dtif';
import type { Composition } from '@dyn/comp-svg-builder';
import { AdvancedInput } from '@dyn/ui';
import { getJsonFunctionExecutionEnv } from '@dyn/utils';

import { runJsonFunction } from '../run-json-function';

export const NumberInput: React.FC<TProps> = (props) => {
	const { composition, field } = props;
	const [value, setValue] = React.useState<number>(field.inputVariant.default);
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

			// eslint-disable-next-line @typescript-eslint/no-floating-promises -- ok
			(async () => {
				const processedActions = await applyModifications(
					field,
					{
						[field.key]: value
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
		[value, field, composition]
	);

	return (
		<fieldset className="w-full rounded-lg border p-4">
			<legend className="-ml-1 px-1 text-sm font-medium">{field.displayName}</legend>
			<AdvancedInput
				childrenAfter={<div />}
				defaultValue={value}
				max={field.inputVariant.max}
				min={field.inputVariant.min}
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
	field: TModificationField<string, TNumberModificationInput>;
}
