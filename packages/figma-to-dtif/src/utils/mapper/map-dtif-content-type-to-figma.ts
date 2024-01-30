import type { COMP } from '@dyn/dtif';

export function mapDTIFContentTypeToFigma(
	dtifContentType: COMP.ContentType
): 'JPG' | 'PNG' | 'SVG' | 'PDF' {
	switch (dtifContentType) {
		case 'JPEG':
			return 'JPG';
		case 'PNG':
			return 'PNG';
		case 'SVG':
			return 'SVG';
	}
}
