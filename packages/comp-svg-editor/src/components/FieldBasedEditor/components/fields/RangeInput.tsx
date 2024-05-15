import React from 'react';
import {
	applyModifications,
	type TModificationField,
	type TRangeModificationInput
} from '@dyn/comp-dtif';
import type { Composition } from '@dyn/comp-svg-builder';
import { Slider } from '@dyn/ui';

export const RangeInput: React.FC<TProps> = (props) => {
	const { composition, field } = props;
	const [value, setValue] = React.useState<number[]>([field.inputType.default]);
	const [error, setError] = React.useState<string | null>(null);

	const handleValueChange = React.useCallback(
		(newValue: number[]) => {
			setValue(newValue);
			setError(null);

			const processedActions = applyModifications(field, {
				[field.key]: newValue[0] ?? 0
			});

			for (const processedAction of processedActions) {
				if (processedAction.resolved) {
					composition.emitInputEvents('Dtif', processedAction.events);
					composition.update();
				} else {
					setError(processedAction.notMetConditions[0]?.message ?? null);
				}
			}
		},
		[field, composition]
	);

	return (
		<fieldset className="w-full rounded-lg border p-4">
			<legend className="-ml-1 px-1 text-sm font-medium">{field.displayName}</legend>
			<Slider
				max={field.inputType.max}
				min={field.inputType.min}
				onValueChange={handleValueChange}
				step={field.inputType.step ?? 1}
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
