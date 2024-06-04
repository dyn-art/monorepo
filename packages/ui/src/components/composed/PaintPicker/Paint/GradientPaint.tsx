'use client';

import { extractStartEndPointFromMat3, rgbaToRgb, shortId, type TVec2 } from '@ibg/utils';
import React from 'react';

import type { TGradientPaint } from '../types';

export const GradientPaint: React.FC<TProps> = (props) => {
	const {
		paint: { variant, stops, opacity },
		size,
		...other
	} = props;
	const gradientId = React.useMemo(() => `gradient-${shortId()}`, []);
	const gradientTransform = React.useMemo(
		() => (variant.transform ? `matrix(${variant.transform.join(' ')})` : undefined),
		[variant.transform]
	);

	const gradientStartStop = React.useMemo(
		() =>
			extractStartEndPointFromMat3(
				size,
				variant.transform ?? [
					[1, 0, 0],
					[0, 1, 0],
					[0, 0, 1]
				]
			),
		[variant.transform, size]
	);
	if (gradientStartStop == null) {
		return null;
	}

	const GradientStops = stops.map((stop, index) => {
		const { rgb, alpha } = rgbaToRgb(stop.color);
		const color = `rgb(${rgb.join(',')})`;
		return (
			<stop
				key={`${index}-${color}`}
				offset={`${stop.position * 100}%`}
				stopColor={color}
				stopOpacity={alpha}
			/>
		);
	});

	let GradientElement;
	switch (variant.type) {
		case 'Linear':
			GradientElement = (
				<linearGradient
					gradientTransform={gradientTransform}
					gradientUnits="userSpaceOnUse"
					id={gradientId}
					x1={gradientStartStop[0][0]}
					x2={gradientStartStop[1][0]}
					y1={gradientStartStop[0][1]}
					y2={gradientStartStop[1][1]}
				>
					{GradientStops}
				</linearGradient>
			);
			break;
		case 'Radial':
			GradientElement = (
				<radialGradient
					gradientTransform={gradientTransform}
					gradientUnits="userSpaceOnUse"
					id={gradientId}
				>
					{GradientStops}
				</radialGradient>
			);
			break;
	}

	return (
		<svg {...other} height={size[1]} width={size[0]}>
			<defs>{GradientElement}</defs>
			<rect fill={`url(#${gradientId})`} height={size[1]} opacity={opacity} width={size[0]} />
		</svg>
	);
};

interface TProps extends React.SVGAttributes<SVGSVGElement> {
	paint: TGradientPaint;
	size: TVec2;
}
