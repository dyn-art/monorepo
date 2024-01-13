import { beforeEach, describe, expect, it } from 'vitest';
import type { DTIFComposition } from '@/rust/dyn_svg_composition_api/bindings';

import { createSVGComposition, mat3, vec3 } from '../core';
import { initWasm } from '../wasm';

export const COMPOSITION_WITH_ONE_RECT = (width: number, height: number): DTIFComposition => ({
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
	},
	paints: {},
	changes: [
		{
			type: 'EntityMoved',
			entity: 1,
			dx: 100,
			dy: -300
		}
	]
});

describe('SVGComposition class tests', () => {
	beforeEach(async () => {
		await initWasm();
	});

	it('should create Composition', async () => {
		const composition = createSVGComposition({
			dtif: COMPOSITION_WITH_ONE_RECT(500, 500),
			width: 500,
			height: 500
		});
		composition.update();
		expect(composition).toBeDefined();
	});
});
