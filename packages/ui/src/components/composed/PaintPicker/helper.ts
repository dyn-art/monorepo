import type { TPaint } from './types';

export function getPaintOpacity(paint: TPaint): number {
	switch (paint.type) {
		case 'Solid':
			return paint.color[3];
		case 'Gradient':
		case 'Image':
			return paint.opacity;
	}
}
