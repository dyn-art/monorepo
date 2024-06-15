import type { ARB } from '@dyn/arb-dtif';

import type { TFigmaFormat } from '../../types';

export function mapFigmaFormatToDtifContentType(
	format: TFigmaFormat,
	width: number,
	height: number
): ARB.AssetContentType {
	switch (format) {
		case 'JPG':
			return { type: 'Jpeg' };
		case 'PNG':
			return { type: 'Png' };
		case 'SVG':
			return { type: 'Svg', width, height };
		default:
			return { type: 'Unknown' };
	}
}
