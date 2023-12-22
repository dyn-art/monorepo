import { DTIFComposition, FontWithContent } from '@dyn/svg-composition';

import { ABEEZEE_ITALIC, INTER_REGULAR } from '../../font';
import { createTransformMatrix } from '../../utils';

export const TEST_DATA = (
	width: number,
	height: number,
	fonts: Record<string, FontWithContent>
): DTIFComposition => {
	return COMPOSITION_FROM_JSON(width, height, fonts);
};

const COMPOSITION_FROM_JSON = (
	width: number,
	height: number,
	fonts: Record<string, FontWithContent>
): DTIFComposition => {
	const dtif = `{"version":"1.0","name":"Frame 3","width":500,"height":500,"nodes":{"0":{"type":"Frame","node":{"name":"Frame 3","node_type":"Frame"},"frame":{"clipContent":false},"compositionMixin":{"isLocked":false,"isVisible":true},"children":[1,3,5,7,9],"dimension":{"height":500,"width":500},"relativeTransform":[1,0,0,0,1,0,0,0,1],"rectangleCornerMixin":{"bottomLeftRadius":0,"bottomRightRadius":0,"topLeftRadius":0,"topRightRadius":0},"blendMixin":{"blendMode":"Normal","opacity":1,"isMask":false},"fill":{"paintIds":[]}},"1":{"type":"Rectangle","node":{"name":"0","node_type":"Rectangle"},"compositionMixin":{"isLocked":false,"isVisible":true},"dimension":{"height":500,"width":500},"relativeTransform":[1,0,0,0,1,0,0,0,1],"rectangleCornerMixin":{"bottomLeftRadius":0,"bottomRightRadius":0,"topLeftRadius":0,"topRightRadius":0},"blendMixin":{"blendMode":"Normal","opacity":1,"isMask":false},"fill":{"paintIds":[2]}},"3":{"type":"Rectangle","node":{"name":"2","node_type":"Rectangle"},"compositionMixin":{"isLocked":false,"isVisible":true},"dimension":{"height":100,"width":100},"relativeTransform":[1,0,0,0,1,0,200,200,1],"rectangleCornerMixin":{"bottomLeftRadius":0,"bottomRightRadius":0,"topLeftRadius":0,"topRightRadius":0},"blendMixin":{"blendMode":"Normal","opacity":1,"isMask":false},"fill":{"paintIds":[4]}},"5":{"type":"Rectangle","node":{"name":"3","node_type":"Rectangle"},"compositionMixin":{"isLocked":false,"isVisible":true},"dimension":{"height":100,"width":100},"relativeTransform":[1,0,0,0,1,0,184,180,1],"rectangleCornerMixin":{"bottomLeftRadius":0,"bottomRightRadius":0,"topLeftRadius":0,"topRightRadius":0},"blendMixin":{"blendMode":"Normal","opacity":1,"isMask":false},"fill":{"paintIds":[6]}},"7":{"type":"Rectangle","node":{"name":"4","node_type":"Rectangle"},"compositionMixin":{"isLocked":false,"isVisible":true},"dimension":{"height":100,"width":100},"relativeTransform":[1,0,0,0,1,0,157,150,1],"rectangleCornerMixin":{"bottomLeftRadius":0,"bottomRightRadius":0,"topLeftRadius":0,"topRightRadius":0},"blendMixin":{"blendMode":"Normal","opacity":1,"isMask":false},"fill":{"paintIds":[8]}},"9":{"type":"Rectangle","node":{"name":"5","node_type":"Rectangle"},"compositionMixin":{"isLocked":false,"isVisible":true},"dimension":{"height":100,"width":100},"relativeTransform":[1,0,0,0,1,0,120,115,1],"rectangleCornerMixin":{"bottomLeftRadius":0,"bottomRightRadius":0,"topLeftRadius":0,"topRightRadius":0},"blendMixin":{"blendMode":"Normal","opacity":1,"isMask":false},"fill":{"paintIds":[10]}}},"paints":{"2":{"type":"Solid","blendMode":"Normal","color":[169,169,169],"opacity":1,"isVisible":true},"4":{"type":"Solid","blendMode":"Normal","color":[0,0,139],"opacity":1,"isVisible":true},"6":{"type":"Solid","blendMode":"Normal","color":[139,83,0],"opacity":1,"isVisible":true},"8":{"type":"Solid","blendMode":"Normal","color":[136,0,139],"opacity":1,"isVisible":true},"10":{"type":"Solid","blendMode":"Normal","color":[0,139,97],"opacity":1,"isVisible":true}},"fonts":{},"rootNodeId":0}`;
	const result = JSON.parse(dtif);

	return result;
};

const COMPOSITION_WITH_ONE_RECT = (
	width: number,
	height: number,
	fonts: Record<string, FontWithContent>
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
				paintIds: [5]
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
				paintIds: [5]
			}
		},
		3: {
			type: 'Text',
			text: {
				segments: [
					{
						value: 'Hello there ',
						style: {
							fontId: INTER_REGULAR.id,
							fontSize: 48
						}
					},
					{
						value: 'Jeff',
						style: {
							fontId: ABEEZEE_ITALIC.id,
							fontSize: 70
						}
					},
					{
						value: '! Long line test testtest',
						style: {
							fontId: INTER_REGULAR.id,
							fontSize: 48
						}
					},
					{
						value: 'Extra small',
						style: {
							fontId: INTER_REGULAR.id,
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
			relativeTransform: createTransformMatrix((width - 100) / 4, (height - 100) / 4, 30),
			fill: {
				paintIds: [5]
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
