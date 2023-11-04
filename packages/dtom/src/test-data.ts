import type { DTIFComposition, Mat3, Vec2, Vec3 } from '@/rust/dyn-dtom/bindings';

// Composition dimensions
const width = 800;
const height = 600;

export const EMPTY_COMPOSITION: DTIFComposition = {
	version: '0.0.1',
	name: 'Test',
	width,
	height,
	root_node_id: 0,
	nodes: {
		0: {
			Frame: {
				node: { node_type: 'Frame' },
				frame: { clip_content: true },
				rectangle_corner_mixin: {
					top_left_radius: 0,
					top_right_radius: 0,
					bottom_left_radius: 0,
					bottom_right_radius: 0
				},
				children_mixin: {
					children: []
				},
				composition_mixin: { is_visible: true, is_locked: false },
				layout_mixin: {
					width,
					height,
					relative_transform: mat3(vec3(1, 0, 0), vec3(0, 1, 0), vec3(0, 0, 1))
				},
				blend_mixin: { blend_mode: 'Normal', opacity: 1, is_mask: false }
			}
		}
	}
};

export const COMPOSITION_ONE_RECT: DTIFComposition = {
	version: '0.0.1',
	name: 'Test',
	width,
	height,
	root_node_id: 0,
	nodes: {
		0: {
			Frame: {
				node: { node_type: 'Frame' },
				frame: { clip_content: true },
				rectangle_corner_mixin: {
					top_left_radius: 0,
					top_right_radius: 0,
					bottom_left_radius: 0,
					bottom_right_radius: 0
				},
				children_mixin: {
					children: [1]
				},
				composition_mixin: { is_visible: true, is_locked: false },
				layout_mixin: {
					width,
					height,
					relative_transform: mat3(vec3(1, 0, 0), vec3(0, 1, 0), vec3(0, 0, 1))
				},
				blend_mixin: { blend_mode: 'Normal', opacity: 1, is_mask: false }
			}
		},
		1: {
			Rectangle: {
				node: { node_type: 'Rectangle' },
				recangle: null,
				rectangle_corner_mixin: {
					top_left_radius: 0,
					top_right_radius: 0,
					bottom_left_radius: 0,
					bottom_right_radius: 0
				},
				composition_mixin: { is_visible: true, is_locked: false },
				layout_mixin: {
					width: 100,
					height: 100,
					relative_transform: mat3(
						vec3(1, 0, 0),
						vec3(0, 1, 0),
						vec3((width - 100) / 2, (height - 100) / 2, 1)
					)
				},
				blend_mixin: { blend_mode: 'Normal', opacity: 1, is_mask: false }
			}
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
