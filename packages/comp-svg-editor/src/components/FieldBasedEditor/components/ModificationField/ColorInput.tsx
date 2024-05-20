import React from 'react';
import {
	applyModifications,
	type TColorModificationInput,
	type TModificationField
} from '@dyn/comp-dtif';
import type { Composition } from '@dyn/comp-svg-builder';
import { PaintPicker, type TPaint } from '@dyn/ui';

import { deterimeJsonFunctionExecutionEnv } from '../determine-json-function-execution-env';
import { runJsonFunction } from '../run-json-function';

export const ColorInput: React.FC<TProps> = (props) => {
	const { composition, field } = props;
	const [value, setValue] = React.useState<TPaint>({
		type: 'Solid',
		color: field.inputVariant.default
	});
	const [error, setError] = React.useState<string | null>(null);

	const onPaintUpdate = React.useCallback(
		(paint: TPaint) => {
			setValue(paint);
			setError(null);

			if (paint.type !== 'Solid') {
				return;
			}

			// eslint-disable-next-line @typescript-eslint/no-floating-promises -- ok
			(async () => {
				// eslint-disable-next-line @typescript-eslint/await-thenable -- idk
				const processedActions = await applyModifications(
					field,
					{
						[field.key]: paint.color
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
			<PaintPicker onPaintUpdate={onPaintUpdate} paint={value} tabs={['Solid']} />
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
	field: TModificationField<string, TColorModificationInput>;
}
