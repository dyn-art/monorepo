import React from 'react';
import type { TArgsMapType, TModificationScript, TPositionModificationInput } from '@dyn/comp-dtif';
import type { Composition } from '@dyn/comp-svg-builder';
import { AdvancedInput } from '@dyn/ui';

export const PositionInput: React.FC<TProps> = (props) => {
	const { composition, script } = props;
	const [xValue, setXValue] = React.useState<number>(script.inputVariant.default.x);
	const [yValue, setYValue] = React.useState<number>(script.inputVariant.default.y);
	const [error, setError] = React.useState<string | null>(null);

	const onXChange = React.useCallback((e: React.ChangeEvent<HTMLInputElement>) => {
		setXValue(parseFloat(e.target.value));
	}, []);

	const onYChange = React.useCallback((e: React.ChangeEvent<HTMLInputElement>) => {
		setYValue(parseFloat(e.target.value));
	}, []);

	const onFocus = React.useCallback(
		(focus: boolean) => {
			if (focus) {
				return;
			}
			setError(null);

			const argsMap: TArgsMapType<TPositionModificationInput> = { x: xValue, y: yValue };
			const scriptError = composition.runScript({
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
				composition.update();
			}
		},
		[composition, xValue, yValue, script.id]
	);

	return (
		<fieldset className="w-full rounded-lg border p-4">
			<legend className="-ml-1 px-1 text-sm font-medium">{script.displayName}</legend>
			<div className="grid grid-cols-2 gap-4">
				<AdvancedInput
					childrenAfter={<div />}
					className="pl-7"
					defaultValue={xValue}
					id="x"
					max={script.inputVariant.max?.[0]}
					min={script.inputVariant.min?.[0]}
					onBlur={() => {
						onFocus(false);
					}}
					onChange={onXChange}
					onFocus={() => {
						onFocus(true);
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
					max={script.inputVariant.max?.[1]}
					min={script.inputVariant.min?.[1]}
					onBlur={() => {
						onFocus(false);
					}}
					onChange={onYChange}
					onFocus={() => {
						onFocus(true);
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
	script: TModificationScript<TPositionModificationInput>;
}
