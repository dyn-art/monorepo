import { DTIFComposition, mat3, vec3 } from '@dyn/svg-composition';

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
				width: 500,
				height: 500
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
			relativeTransform: createTransformMatrix((500 - 100) / 2, (500 - 100) / 2, 30),
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
			relativeTransform: createTransformMatrix((500 - 100) / 2, (500 - 100) / 2, 0),
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
	}
});

function createTransformMatrix(x: number, y: number, angleDegrees: number) {
	const angleRadians = (angleDegrees * Math.PI) / 180; // Convert angle to radians

	return mat3(
		vec3(Math.cos(angleRadians), -Math.sin(angleRadians), 0),
		vec3(Math.sin(angleRadians), Math.cos(angleRadians), 0),
		vec3(x, y, 1)
	);
}
