import { DTIFComposition } from '@dyn/svg-composition';

import { createTransformMatrix } from '../../utils';

export const COMPOSITION_WITH_ONE_RECT = (width: number, height: number): DTIFComposition => ({
	version: '0.0.1',
	name: 'Test',
	width,
	height,
	rootNodeId: 0,
	nodes: {
		0: {
			type: 'Frame',
			children: [1, 2],
			dimension: {
				width,
				height
			},
			relativeTransform: createTransformMatrix(0, 0, 0),
			fill: {
				paints: [10]
			}
		},
		1: {
			type: 'Rectangle',
			compositionMixin: { isVisible: true, isLocked: false },
			dimension: {
				width: 100,
				height: 100
			},
			relativeTransform: createTransformMatrix((width - 100) / 2, (height - 100) / 2, 30),
			fill: {
				paints: [11]
			}
		},
		2: {
			type: 'Rectangle',
			compositionMixin: { isVisible: true, isLocked: false },
			dimension: {
				width: 100,
				height: 100
			},
			relativeTransform: createTransformMatrix((width - 100) / 2, (height - 100) / 2, 0),
			fill: {
				paints: [12]
			}
		}
	},
	paints: {
		10: {
			type: 'Solid',
			blendMode: 'Normal',
			color: [169, 169, 169],
			isVisible: true,
			opacity: 1
		},
		11: {
			type: 'Solid',
			blendMode: 'Normal',
			color: [255, 0, 0],
			isVisible: true,
			opacity: 1
		},
		12: {
			type: 'Solid',
			blendMode: 'Normal',
			color: [0, 0, 139],
			isVisible: true,
			opacity: 1
		}
	},
	fonts: []
});
