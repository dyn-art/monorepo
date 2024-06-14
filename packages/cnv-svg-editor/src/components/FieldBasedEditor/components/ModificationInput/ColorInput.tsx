import React from 'react';
import type { TArgsMapType, TColorModificationInput, TModificationScript } from '@dyn/cnv-dtif';
import type { Canvas } from '@dyn/cnv-svg-builder';
import { PaintPicker, type TPaint } from '@dyn/ui';

export const ColorInput: React.FC<TProps> = (props) => {
	const { canvas, script } = props;
	const [value, setValue] = React.useState<TPaint>({
		type: 'Solid',
		color: [
			script.inputVariant.default.r,
			script.inputVariant.default.g,
			script.inputVariant.default.b,
			script.inputVariant.default.a
		]
	});
	const [error, setError] = React.useState<string | null>(null);

	const onPaintUpdate = React.useCallback(
		(paint: TPaint) => {
			setValue(paint);
			setError(null);

			if (paint.type !== 'Solid') {
				return;
			}

			const argsMap: TArgsMapType<TColorModificationInput> = {
				r: paint.color[0],
				g: paint.color[1],
				b: paint.color[2],
				a: paint.color[3]
			};
			const scriptError = canvas.executeScript({
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
				canvas.update();
			}
		},
		[canvas, script.id]
	);

	return (
		<fieldset className="w-full rounded-lg border p-4">
			<legend className="-ml-1 px-1 text-sm font-medium">{script.displayName}</legend>
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
	canvas: Canvas;
	script: TModificationScript<TColorModificationInput>;
}
