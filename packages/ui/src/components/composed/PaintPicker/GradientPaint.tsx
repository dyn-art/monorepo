import React from 'react';
import { shortId } from '@dyn/utils';

import type { TGradientPaint } from './types';

export const GradientPaint: React.FC<TProps> = (props) => {
	const {
		paint: { variant, stops }
	} = props;
	const gradientId = React.useMemo(() => `gradient-${shortId()}`, []);
	const gradientTransform = React.useMemo(
		() => (variant.transform ? `matrix(${variant.transform.join(' ')})` : undefined),
		[variant.transform]
	);

	const GradientStops = stops.map((stop, index) => {
		const color = `rgb(${stop.color.join(',')})`;
		return (
			<stop
				key={`${index}-${color}`}
				offset={`${stop.position * 100}%`}
				stopColor={color}
				stopOpacity={stop.opacity ?? 1}
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
		<svg height="100%" width="100%">
			<defs>{GradientElement}</defs>
			<rect fill={`url(#${gradientId})`} height="100%" width="100%" />
		</svg>
	);
};

interface TProps {
	paint: TGradientPaint;
}
