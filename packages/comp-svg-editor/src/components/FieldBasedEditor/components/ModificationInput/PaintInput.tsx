import { arrayToMat3, mat3ToArray, rgbaToRgb, rgbToRgba } from '@ibg/utils';
import React from 'react';
import type {
	COMP,
	TArgsMapType,
	TModificationScript,
	TPaintModificationInput,
	TPaintModificationReturnType
} from '@dyn/comp-dtif';
import type { Composition } from '@dyn/comp-svg-builder';
import {
	PaintPicker,
	type TGradientColorStop,
	type TGradientVariant,
	type TImageScaleMode,
	type TPaint
} from '@dyn/ui';

export const PaintInput: React.FC<TProps> = (props) => {
	const { composition, script } = props;
	const [value, setValue] = React.useState<TPaint>(
		mapCompToPaint(script.inputVariant.default.paint, script.inputVariant.default.opacity)
	);
	const [error, setError] = React.useState<string | null>(null);

	const onPaintUpdate = React.useCallback(
		(paint: TPaint) => {
			setValue(paint);
			setError(null);

			const argsMap: TArgsMapType<TPaintModificationInput> = mapPaintToComp(paint);
			const scriptError = composition.executeScript({
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
				composition.update();
			}
		},
		[composition, script.id]
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
	composition: Composition;
	script: TModificationScript<TPaintModificationInput>;
}

function mapPaintToComp(paint: TPaint): TPaintModificationReturnType {
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
		case 'Image':
			return {
				paint: {
					type: 'Image',
					scaleMode: mapImageScaleModeToComp(paint.scaleMode),
					imageId: { type: 'ReferenceId', referenceId: 'unknown' }
				},
				opacity: paint.opacity,
				content: paint.content
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

function mapImageScaleModeToComp(imageScaleMode: TImageScaleMode): COMP.ImageScaleMode {
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

function mapCompToPaint(compPaint: COMP.Paint, opacity = 1, content: number[] = []): TPaint {
	switch (compPaint.type) {
		case 'Solid':
			return {
				type: 'Solid',
				color: rgbToRgba(compPaint.color, opacity)
			};
		case 'Gradient':
			return {
				type: 'Gradient',
				variant: mapCompGradientVariantToPaint(compPaint.variant),
				stops: compPaint.stops.map(mapCompGradientColorStopToPaint),
				opacity
			};
		case 'Image':
			return {
				type: 'Image',
				scaleMode:
					compPaint.scaleMode != null
						? mapCompImageScaleModeToPaint(compPaint.scaleMode)
						: { type: 'Fill' },
				content,
				opacity
			};
	}
}

function mapCompGradientVariantToPaint(
	compGradientVariant: COMP.GradientVariant
): TGradientVariant {
	switch (compGradientVariant.type) {
		case 'Linear':
			return {
				type: 'Linear',
				transform:
					compGradientVariant.transform != null
						? arrayToMat3(compGradientVariant.transform) ?? undefined
						: undefined
			};
		case 'Radial':
			return {
				type: 'Radial',
				transform:
					compGradientVariant.transform != null
						? arrayToMat3(compGradientVariant.transform) ?? undefined
						: undefined
			};
	}
}

function mapCompGradientColorStopToPaint(
	compGradientColorStop: COMP.GradientColorStop
): TGradientColorStop {
	return {
		position: compGradientColorStop.position,
		color: rgbToRgba(compGradientColorStop.color, compGradientColorStop.opacity || 1)
	};
}

function mapCompImageScaleModeToPaint(compImageScaleMode: COMP.ImageScaleMode): TImageScaleMode {
	switch (compImageScaleMode.type) {
		case 'Fill':
			return { type: 'Fill' };
		case 'Fit':
			return { type: 'Fit' };
		case 'Crop':
			return {
				type: 'Crop',
				transform:
					compImageScaleMode.transform != null
						? arrayToMat3(compImageScaleMode.transform) ?? undefined
						: undefined
			};
		case 'Tile':
			return {
				type: 'Tile',
				scalingFactor: compImageScaleMode.scalingFactor,
				rotation: compImageScaleMode.rotation
			};
	}
}
