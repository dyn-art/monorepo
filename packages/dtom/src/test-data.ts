import type { DTIFComposition, Mat3, Vec2, Vec3 } from '@/rust/dyn_composition_api/bindings';

// Composition dimensions
const width = 800;
const height = 600;

export const EMPTY_COMPOSITION: DTIFComposition = {
	version: '0.0.1',
	name: 'Test',
	width,
	height,
	rootNodeId: 0,
	nodes: {
		0: {
			type: 'Frame',
			frame: { clip_content: true },
			children: [],
			compositionMixin: { isVisible: true, isLocked: false },
			dimension: {
				width,
				height
			},
			relativeTransform: mat3(vec3(1, 0, 0), vec3(0, 1, 0), vec3(0, 0, 1)),
			blendMixin: { blendMode: 'Normal', opacity: 1, isMask: false }
		}
	}
};

export const COMPOSITION_ONE_RECT: DTIFComposition = {
	version: '0.0.1',
	name: 'Test',
	width,
	height,
	rootNodeId: 0,
	nodes: {
		0: {
			type: 'Frame',
			frame: { clip_content: true },
			children: [1],
			compositionMixin: { isVisible: true, isLocked: false },
			dimension: {
				width,
				height
			},
			relativeTransform: mat3(vec3(1, 0, 0), vec3(0, 1, 0), vec3(0, 0, 1)),
			blendMixin: { blendMode: 'Normal', opacity: 1, isMask: false }
		},
		1: {
			type: 'Rectangle',
			compositionMixin: { isVisible: true, isLocked: false },
			dimension: {
				width: 100,
				height: 100
			},
			relativeTransform: mat3(
				vec3(1, 0, 0),
				vec3(0, 1, 0),
				vec3((width - 100) / 2, (height - 100) / 2, 1)
			),
			blendMixin: { blendMode: 'Normal', opacity: 1, isMask: false }
		}
	}
};

// =============================================================================
// Helper
// =============================================================================

function vec2(x: number, y: number): Vec2 {
	return [x, y];
}

function vec3(x: number, y: number, z: number): Vec3 {
	return [x, y, z];
}

function mat3(xAxis: Vec3, yAxis: Vec3, zAxis: Vec3): Mat3 {
	return [...xAxis, ...yAxis, ...zAxis];
}
