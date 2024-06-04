import type { TVec2 } from '@ibg/utils';

import type { TSolidPaint } from '../types';

export const SolidPaint: React.FC<TProps> = (props) => {
	const {
		paint: {
			color: [r, g, b, alpha]
		},
		size,
		...other
	} = props;

	return (
		<div
			{...other}
			style={{
				...other.style,
				background: `rgb(${r}, ${g}, ${b})`,
				opacity: alpha,
				width: size[0],
				height: size[1]
			}}
		/>
	);
};

interface TProps extends React.HTMLAttributes<HTMLDivElement> {
	paint: TSolidPaint;
	size: TVec2;
}
