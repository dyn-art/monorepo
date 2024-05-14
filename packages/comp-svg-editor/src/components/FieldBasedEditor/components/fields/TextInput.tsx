import React from 'react';
import {
	applyModifications,
	type TModificationField,
	type TTextModificationInput
} from '@dyn/comp-dtif';
import type { Composition } from '@dyn/comp-svg-builder';
import { AdvancedInput } from '@dyn/ui';

export const TextInput: React.FC<TProps> = (props) => {
	const { composition, field } = props;
	const [value, setValue] = React.useState<string>(field.inputType.default);
	const [error, setError] = React.useState<string | null>(null);

	const handleChange = React.useCallback((e: React.ChangeEvent<HTMLInputElement>) => {
		// TODO: \n gets escaped to \\n
		setValue(e.target.value);
	}, []);

	const handleFocus = React.useCallback(
		(focus: boolean) => {
			if (focus) {
				return;
			}
			setError(null);

			const processedActions = applyModifications(field, {
				[field.key]: value
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
		[value, field, composition]
	);

	return (
		<fieldset className="w-full rounded-lg border p-4">
			<legend className="-ml-1 px-1 text-sm font-medium">{field.displayName}</legend>
			<AdvancedInput
				childrenAfter={<div />}
				defaultValue={value}
				onBlur={() => {
					handleFocus(false);
				}}
				onChange={handleChange}
				onFocus={() => {
					handleFocus(true);
				}}
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
	field: TModificationField<string, TTextModificationInput>;
}
