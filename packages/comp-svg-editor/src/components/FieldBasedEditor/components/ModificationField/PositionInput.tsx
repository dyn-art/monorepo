import React from 'react';
import {
	applyModifications,
	type TModificationField,
	type TPositionModificationInput
} from '@dyn/comp-dtif';
import type { Composition } from '@dyn/comp-svg-builder';
import { AdvancedInput } from '@dyn/ui';

export const PositionInput: React.FC<TProps> = (props) => {
	const { composition, field } = props;
	const [xValue, setXValue] = React.useState<number>(field.inputType.default[0]);
	const [yValue, setYValue] = React.useState<number>(field.inputType.default[1]);
	const [error, setError] = React.useState<string | null>(null);

	const handleXChange = React.useCallback((e: React.ChangeEvent<HTMLInputElement>) => {
		setXValue(parseFloat(e.target.value));
	}, []);

	const handleYChange = React.useCallback((e: React.ChangeEvent<HTMLInputElement>) => {
		setYValue(parseFloat(e.target.value));
	}, []);

	const handleFocus = React.useCallback(
		(focus: boolean) => {
			if (focus) {
				return;
			}
			setError(null);

			const processedActions = applyModifications(field, {
				[field.key]: [xValue, yValue]
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
		[xValue, yValue, field, composition]
	);

	return (
		<fieldset className="w-full rounded-lg border p-4">
			<legend className="-ml-1 px-1 text-sm font-medium">{field.displayName}</legend>
			<div className="grid grid-cols-2 gap-4">
				<AdvancedInput
					childrenAfter={<div />}
					className="pl-7"
					defaultValue={xValue}
					id="x"
					max={field.inputType.max?.[0]}
					min={field.inputType.min?.[0]}
					onBlur={() => {
						handleFocus(false);
					}}
					onChange={handleXChange}
					onFocus={() => {
						handleFocus(true);
					}}
					type="number"
					variant={error != null ? 'destructive' : 'default'}
				>
					<div className="pointer-events-none absolute inset-y-0 left-0 flex items-center pl-3">
						<p className="text-gray-400">x</p>
					</div>
				</AdvancedInput>

				<AdvancedInput
					childrenAfter={<div />}
					className="pl-7"
					defaultValue={yValue}
					id="y"
					max={field.inputType.max?.[1]}
					min={field.inputType.min?.[1]}
					onBlur={() => {
						handleFocus(false);
					}}
					onChange={handleYChange}
					onFocus={() => {
						handleFocus(true);
					}}
					type="number"
					variant={error != null ? 'destructive' : 'default'}
				>
					<div className="pointer-events-none absolute inset-y-0 left-0 flex items-center pl-3">
						<p className="text-gray-400">y</p>
					</div>
				</AdvancedInput>
			</div>
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
	field: TModificationField<string, TPositionModificationInput>;
}
