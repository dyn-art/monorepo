import React from 'react';
import {
	applyModifications,
	type COMP,
	type TModificationField,
	type TPaintModificationInput
} from '@dyn/comp-dtif';
import type { Composition } from '@dyn/comp-svg-builder';
import { PaintPicker, type TGradientColorStop, type TGradientVariant, type TPaint } from '@dyn/ui';
import {
	arrayToMat3,
	getJsonFunctionExecutionEnv,
	mat3ToArray,
	rgbaToRgb,
	rgbToRgba
} from '@dyn/utils';

import { runJsonFunction } from '../run-json-function';

export const PaintInput: React.FC<TProps> = (props) => {
	const { composition, field } = props;
	const [value, setValue] = React.useState<TPaint>(
		mapCompToPaint(field.inputVariant.default.paint, field.inputVariant.default.opacity)
	);
	const [error, setError] = React.useState<string | null>(null);

	const onPaintUpdate = React.useCallback(
		(paint: TPaint) => {
			setValue(paint);
			setError(null);

			console.log({ paint: mapPaintToComp(paint) });

			// eslint-disable-next-line @typescript-eslint/no-floating-promises -- ok
			(async () => {
				const processedActions = await applyModifications(
					field,
					{
						[field.key]: mapPaintToComp(paint)
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
			{/* TODO: PaintPicker (InputRow) takes full width although its children don't take full width 
			and thus blocks tapping out of the popover at some unwanted places */}
			<PaintPicker onPaintUpdate={onPaintUpdate} paint={value} tabs={['Solid', 'Gradient']} />
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
	field: TModificationField<string, TPaintModificationInput>;
}

function mapPaintToComp(paint: TPaint): { paint: COMP.Paint; opacity: number } {
	switch (paint.type) {
		case 'Solid': {
			const { rgb, alpha } = rgbaToRgb(paint.color);
			return {
				paint: {
					type: 'Solid',
					color: rgb
				},
				opacity: alpha
			};
		}
		case 'Gradient':
			return {
				paint: {
					type: 'Gradient',
					variant: mapGradientVariantToComp(paint.variant),
					stops: paint.stops.map(mapGradientColorStopToComp)
				},
				opacity: paint.opacity
			};
	}
}

function mapGradientVariantToComp(gradientVariant: TGradientVariant): COMP.GradientVariant {
	switch (gradientVariant.type) {
		case 'Linear':
			return {
				type: 'Linear',
				transform:
					gradientVariant.transform != null
						? mat3ToArray(gradientVariant.transform)
						: [1, 0, 0, 0, 1, 0, 0, 0, 1]
			};
		case 'Radial':
			return {
				type: 'Radial',
				transform:
					gradientVariant.transform != null
						? mat3ToArray(gradientVariant.transform)
						: [1, 0, 0, 0, 1, 0, 0, 0, 1]
			};
	}
}

function mapGradientColorStopToComp(gradientColorStop: TGradientColorStop): COMP.GradientColorStop {
	const { rgb, alpha } = rgbaToRgb(gradientColorStop.color);
	return {
		position: gradientColorStop.position,
		color: rgb,
		opacity: alpha
	};
}

function mapCompToPaint(compPaint: COMP.Paint, opacity = 1): TPaint {
	switch (compPaint.type) {
		case 'Solid':
			return {
				type: 'Solid',
				color: rgbToRgba(compPaint.color, opacity)
			};

		case 'Gradient':
			return {
				type: 'Gradient',
				variant: mapCompGradientVariantToTPaint(compPaint.variant),
				stops: compPaint.stops.map(mapCompGradientColorStopToTPaint),
				opacity
			};

		case 'Image':
			return {
				type: 'Solid',
				color: [0, 0, 0, 1]
			};
	}
}

function mapCompGradientVariantToTPaint(gradientVariant: COMP.GradientVariant): TGradientVariant {
	switch (gradientVariant.type) {
		case 'Linear':
			return {
				type: 'Linear',
				transform:
					gradientVariant.transform != null
						? arrayToMat3(gradientVariant.transform) ?? undefined
						: undefined
			};
		case 'Radial':
			return {
				type: 'Radial',
				transform:
					gradientVariant.transform != null
						? arrayToMat3(gradientVariant.transform) ?? undefined
						: undefined
			};
	}
}

function mapCompGradientColorStopToTPaint(
	gradientColorStop: COMP.GradientColorStop
): TGradientColorStop {
	return {
		position: gradientColorStop.position,
		color: rgbToRgba(gradientColorStop.color, gradientColorStop.opacity || 1)
	};
}
