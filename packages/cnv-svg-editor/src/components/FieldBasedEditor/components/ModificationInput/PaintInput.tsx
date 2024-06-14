import { arrayToMat3, mat3ToArray, rgbaToRgb, rgbToRgba } from '@ibg/utils';
import React from 'react';
import type {
	CNV,
	TArgsMapType,
	TModificationScript,
	TPaintModificationInput,
	TPaintModificationReturnType
} from '@dyn/cnv-dtif';
import type { Canvas } from '@dyn/cnv-svg-builder';
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
		mapCnvToPaint(script.inputVariant.default.paint, script.inputVariant.default.opacity)
	);
	const [error, setError] = React.useState<string | null>(null);

	const onPaintUpdate = React.useCallback(
		(paint: TPaint) => {
			setValue(paint);
			setError(null);

			const argsMap: TArgsMapType<TPaintModificationInput> = mapPaintToCnv(paint);
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
	canvas: Canvas;
	script: TModificationScript<TPaintModificationInput>;
}

function mapPaintToCnv(paint: TPaint): TPaintModificationReturnType {
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
					variant: mapGradientVariantToCnv(paint.variant),
					stops: paint.stops.map(mapGradientColorStopToCnv)
				},
				opacity: paint.opacity
			};
		case 'Image':
			return {
				paint: {
					type: 'Image',
					scaleMode: mapImageScaleModeToCnv(paint.scaleMode),
					imageId: { type: 'ReferenceId', referenceId: 'unknown' }
				},
				opacity: paint.opacity,
				content: paint.content
			};
	}
}

function mapGradientVariantToCnv(gradientVariant: TGradientVariant): CNV.GradientVariant {
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

function mapGradientColorStopToCnv(gradientColorStop: TGradientColorStop): CNV.GradientColorStop {
	const { rgb, alpha } = rgbaToRgb(gradientColorStop.color);
	return {
		position: gradientColorStop.position,
		color: rgb,
		opacity: alpha
	};
}

function mapImageScaleModeToCnv(imageScaleMode: TImageScaleMode): CNV.ImageScaleMode {
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

function mapCnvToPaint(cnvPaint: CNV.Paint, opacity = 1, content: number[] = []): TPaint {
	switch (cnvPaint.type) {
		case 'Solid':
			return {
				type: 'Solid',
				color: rgbToRgba(cnvPaint.color, opacity)
			};
		case 'Gradient':
			return {
				type: 'Gradient',
				variant: mapCnvGradientVariantToPaint(cnvPaint.variant),
				stops: cnvPaint.stops.map(mapCnvGradientColorStopToPaint),
				opacity
			};
		case 'Image':
			return {
				type: 'Image',
				scaleMode:
					cnvPaint.scaleMode != null
						? mapCnvImageScaleModeToPaint(cnvPaint.scaleMode)
						: { type: 'Fill' },
				content,
				opacity
			};
	}
}

function mapCnvGradientVariantToPaint(cnvGradientVariant: CNV.GradientVariant): TGradientVariant {
	switch (cnvGradientVariant.type) {
		case 'Linear':
			return {
				type: 'Linear',
				transform:
					cnvGradientVariant.transform != null
						? arrayToMat3(cnvGradientVariant.transform) ?? undefined
						: undefined
			};
		case 'Radial':
			return {
				type: 'Radial',
				transform:
					cnvGradientVariant.transform != null
						? arrayToMat3(cnvGradientVariant.transform) ?? undefined
						: undefined
			};
	}
}

function mapCnvGradientColorStopToPaint(
	cnvGradientColorStop: CNV.GradientColorStop
): TGradientColorStop {
	return {
		position: cnvGradientColorStop.position,
		color: rgbToRgba(cnvGradientColorStop.color, cnvGradientColorStop.opacity || 1)
	};
}

function mapCnvImageScaleModeToPaint(cnvImageScaleMode: CNV.ImageScaleMode): TImageScaleMode {
	switch (cnvImageScaleMode.type) {
		case 'Fill':
			return { type: 'Fill' };
		case 'Fit':
			return { type: 'Fit' };
		case 'Crop':
			return {
				type: 'Crop',
				transform:
					cnvImageScaleMode.transform != null
						? arrayToMat3(cnvImageScaleMode.transform) ?? undefined
						: undefined
			};
		case 'Tile':
			return {
				type: 'Tile',
				scalingFactor: cnvImageScaleMode.scalingFactor,
				rotation: cnvImageScaleMode.rotation
			};
	}
}
