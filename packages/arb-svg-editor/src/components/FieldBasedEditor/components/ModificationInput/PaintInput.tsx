import { arrayToMat3, mat3ToArray, rgbaToRgb, rgbToRgba } from '@ibg/utils';
import React from 'react';
import type {
	ARB,
	TArgsMapType,
	TModificationScript,
	TPaintModificationInput,
	TPaintModificationReturnType
} from '@dyn/arb-dtif';
import type { Artboard } from '@dyn/arb-svg-builder';
import {
	PaintPicker,
	type TGradientColorStop,
	type TGradientVariant,
	type TImageScaleMode,
	type TPaint
} from '@dyn/ui';

export const PaintInput: React.FC<TProps> = (props) => {
	const { canvas, script } = props;
	const [value, setValue] = React.useState<TPaint>(
		mapArbToPaint(script.inputVariant.default.paint, script.inputVariant.default.opacity)
	);
	const [error, setError] = React.useState<string | null>(null);

	const onPaintUpdate = React.useCallback(
		(paint: TPaint) => {
			setValue(paint);
			setError(null);

			const argsMap: TArgsMapType<TPaintModificationInput> = mapPaintToArb(paint);
			const scriptError = canvas.executeScript({
				id: script.id,
				argsMap: argsMap as any // TODO: Make typesafe
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
			{/* TODO: PaintPicker (InputRow) takes full width although its children don't take full width 
			and thus blocks tapping out of the popover at some unwanted places */}
			<PaintPicker
				onPaintUpdate={onPaintUpdate}
				paint={value}
				tabs={['Solid', 'Gradient', 'Image']}
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
	canvas: Artboard;
	script: TModificationScript<TPaintModificationInput>;
}

function mapPaintToArb(paint: TPaint): TPaintModificationReturnType {
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
					variant: mapGradientVariantToArb(paint.variant),
					stops: paint.stops.map(mapGradientColorStopToArb)
				},
				opacity: paint.opacity
			};
		case 'Image':
			return {
				paint: {
					type: 'Image',
					scaleMode: mapImageScaleModeToArb(paint.scaleMode),
					imageId: { type: 'ReferenceId', referenceId: 'unknown' }
				},
				opacity: paint.opacity,
				content: paint.content
			};
	}
}

function mapGradientVariantToArb(gradientVariant: TGradientVariant): ARB.GradientVariant {
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

function mapGradientColorStopToArb(gradientColorStop: TGradientColorStop): ARB.GradientColorStop {
	const { rgb, alpha } = rgbaToRgb(gradientColorStop.color);
	return {
		position: gradientColorStop.position,
		color: rgb,
		opacity: alpha
	};
}

function mapImageScaleModeToArb(imageScaleMode: TImageScaleMode): ARB.ImageScaleMode {
	switch (imageScaleMode.type) {
		case 'Fill':
			return { type: 'Fill' };
		case 'Fit':
			return { type: 'Fit' };
		case 'Crop':
			return {
				type: 'Crop',
				transform:
					imageScaleMode.transform != null ? mat3ToArray(imageScaleMode.transform) : undefined
			};
		case 'Tile':
			return {
				type: 'Tile',
				scalingFactor: imageScaleMode.scalingFactor,
				rotation: imageScaleMode.rotation
			};
	}
}

function mapArbToPaint(arbPaint: ARB.Paint, opacity = 1, content: number[] = []): TPaint {
	switch (arbPaint.type) {
		case 'Solid':
			return {
				type: 'Solid',
				color: rgbToRgba(arbPaint.color, opacity)
			};
		case 'Gradient':
			return {
				type: 'Gradient',
				variant: mapArbGradientVariantToPaint(arbPaint.variant),
				stops: arbPaint.stops.map(mapArbGradientColorStopToPaint),
				opacity
			};
		case 'Image':
			return {
				type: 'Image',
				scaleMode:
					arbPaint.scaleMode != null
						? mapArbImageScaleModeToPaint(arbPaint.scaleMode)
						: { type: 'Fill' },
				content,
				opacity
			};
	}
}

function mapArbGradientVariantToPaint(arbGradientVariant: ARB.GradientVariant): TGradientVariant {
	switch (arbGradientVariant.type) {
		case 'Linear':
			return {
				type: 'Linear',
				transform:
					arbGradientVariant.transform != null
						? arrayToMat3(arbGradientVariant.transform) ?? undefined
						: undefined
			};
		case 'Radial':
			return {
				type: 'Radial',
				transform:
					arbGradientVariant.transform != null
						? arrayToMat3(arbGradientVariant.transform) ?? undefined
						: undefined
			};
	}
}

function mapArbGradientColorStopToPaint(
	arbGradientColorStop: ARB.GradientColorStop
): TGradientColorStop {
	return {
		position: arbGradientColorStop.position,
		color: rgbToRgba(arbGradientColorStop.color, arbGradientColorStop.opacity || 1)
	};
}

function mapArbImageScaleModeToPaint(arbImageScaleMode: ARB.ImageScaleMode): TImageScaleMode {
	switch (arbImageScaleMode.type) {
		case 'Fill':
			return { type: 'Fill' };
		case 'Fit':
			return { type: 'Fit' };
		case 'Crop':
			return {
				type: 'Crop',
				transform:
					arbImageScaleMode.transform != null
						? arrayToMat3(arbImageScaleMode.transform) ?? undefined
						: undefined
			};
		case 'Tile':
			return {
				type: 'Tile',
				scalingFactor: arbImageScaleMode.scalingFactor,
				rotation: arbImageScaleMode.rotation
			};
	}
}
