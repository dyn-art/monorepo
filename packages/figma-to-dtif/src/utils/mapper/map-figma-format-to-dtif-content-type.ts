import type { CNV } from '@dyn/cnv-dtif';

import type { TFigmaFormat } from '../../types';

export function mapFigmaFormatToDtifContentType(
	format: TFigmaFormat,
	width: number,
	height: number
): CNV.AssetContentType {
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
