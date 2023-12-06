import { DTIFComposition, FontWithContent } from '@dyn/svg-composition';

import { createTransformMatrix } from '../../utils';
import { ABEEZEE_ITALIC, INTER_REGULAR } from './font';

export const COMPOSITION_WITH_ONE_RECT = (
	width: number,
	height: number,
	fonts: FontWithContent[]
): DTIFComposition => ({
	version: '0.0.1',
	name: 'Test',
	width,
	height,
	rootNodeId: 0,
	nodes: {
		0: {
			type: 'Frame',
			children: [1, 2, 3],
			dimension: {
				width,
				height
			},
			relativeTransform: createTransformMatrix(0, 0, 0)
		},
		1: {
			type: 'Rectangle',
			compositionMixin: { isVisible: true, isLocked: false },
			dimension: {
				width: 100,
				height: 100
			},
			relativeTransform: createTransformMatrix((width - 100) / 2, (height - 100) / 2, 30),
			rectangleCornerMixin: {
				bottomLeftRadius: 20,
				bottomRightRadius: 0,
				topLeftRadius: 0,
				topRightRadius: 0
			},
			fill: {
				paints: [5]
			}
		},
		2: {
			type: 'Rectangle',
			compositionMixin: { isVisible: true, isLocked: false },
			dimension: {
				width: 150,
				height: 150
			},
			relativeTransform: createTransformMatrix((width - 100) / 1.5, (height - 100) / 1.5, 0),
			rectangleCornerMixin: {
				bottomLeftRadius: 10,
				bottomRightRadius: 20,
				topLeftRadius: 40,
				topRightRadius: 80
			},
			fill: {
				paints: [5]
			}
		},
		3: {
			type: 'Text',
			text: {
				sections: [
					{
						value: 'Hello there ',
						style: {
							fontHash: INTER_REGULAR.hash,
							fontSize: 48
						}
					},
					{
						value: 'Jeff',
						style: {
							fontHash: ABEEZEE_ITALIC.hash,
							fontSize: 70
						}
					},
					{
						value: '! Long line test testtest',
						style: {
							fontHash: INTER_REGULAR.hash,
							fontSize: 48
						}
					},
					{
						value: 'Extra small',
						style: {
							fontHash: INTER_REGULAR.hash,
							fontSize: 24
						}
					}
				]
			},
			compositionMixin: { isVisible: true, isLocked: false },
			dimension: {
				width: 500,
				height: 300
			},
			relativeTransform: createTransformMatrix((width - 100) / 4, (height - 100) / 4, 0),
			fill: {
				paints: [5]
			}
		}
	},
	paints: {
		5: {
			type: 'Solid',
			blendMode: 'Normal',
			color: [189, 183, 107],
			isVisible: true,
			opacity: 1
		}
	},
	fonts,
	changes: [
		// {
		// 	type: 'EntityMoved',
		// 	entity: 1,
		// 	dx: 100,
		// 	dy: -300
		// }
	]
});
