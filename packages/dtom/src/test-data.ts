import type { DTIFComposition } from '@/rust/dyn_composition_api/bindings';

import { mat3, vec3 } from './core/helper';

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
			children: [],
			dimension: {
				width,
				height
			},
			relativeTransform: mat3(vec3(1, 0, 0), vec3(0, 1, 0), vec3(0, 0, 1))
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
			children: [1],
			dimension: {
				width,
				height
			},
			relativeTransform: mat3(vec3(1, 0, 0), vec3(0, 1, 0), vec3(0, 0, 1))
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
			)
		}
	}
};

// =============================================================================
// Helper
// =============================================================================
