import React from 'react';
import {
	applyModifications,
	type TModificationField,
	type TRangeModificationInput
} from '@dyn/comp-dtif';
import type { Composition } from '@dyn/comp-svg-builder';
import { Slider } from '@dyn/ui';
import { getJsonFunctionExecutionEnv } from '@dyn/utils';

import { runJsonFunction } from '../run-json-function';

export const RangeInput: React.FC<TProps> = (props) => {
	const { composition, field } = props;
	const [value, setValue] = React.useState<number[]>([field.inputVariant.default]);
	const [error, setError] = React.useState<string | null>(null);

	const onValueChange = React.useCallback(
		(newValue: number[]) => {
			setValue(newValue);
			setError(null);

			// eslint-disable-next-line @typescript-eslint/no-floating-promises -- ok
			(async () => {
				const processedActions = await applyModifications(
					field,
					{
						[field.key]: newValue[0] ?? 0
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
			<Slider
				max={field.inputVariant.max}
				min={field.inputVariant.min}
				onValueChange={onValueChange}
				step={field.inputVariant.step ?? 1}
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
	composition: Composition;
	field: TModificationField<string, TRangeModificationInput>;
}
