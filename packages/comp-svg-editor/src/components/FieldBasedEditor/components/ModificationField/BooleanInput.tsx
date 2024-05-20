import React from 'react';
import {
	applyModifications,
	type TBooleanModificationInput,
	type TModificationField
} from '@dyn/comp-dtif';
import type { Composition } from '@dyn/comp-svg-builder';
import { Switch } from '@dyn/ui';

import { deterimeJsonFunctionExecutionEnv } from '../determine-json-function-execution-env';
import { runJsonFunction } from '../run-json-function';

export const BooleanInput: React.FC<TProps> = (props) => {
	const { composition, field } = props;
	const [value, setValue] = React.useState<boolean>(field.inputVariant.default);
	const [error, setError] = React.useState<string | null>(null);

	const onCheckedChange = React.useCallback(
		(checked: boolean) => {
			setValue(checked);
			setError(null);

			// eslint-disable-next-line @typescript-eslint/no-floating-promises -- ok
			(async () => {
				// eslint-disable-next-line @typescript-eslint/await-thenable -- idk
				const processedActions = await applyModifications(
					field,
					{
						[field.key]: checked
					},
					async (jsonFunction, args) =>
						runJsonFunction(jsonFunction, args, deterimeJsonFunctionExecutionEnv(jsonFunction))
				);

				for (const processedAction of processedActions) {
					if (processedAction.resolved) {
						composition.emitInputEvents('Dtif', processedAction.events);
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
	composition: Composition;
	field: TModificationField<string, TBooleanModificationInput>;
}
