import { beforeEach, describe, expect, it } from 'vitest';

import { createSVGComposition } from '../core/composition';
import { initWasm } from '../wasm';

describe('dtom', () => {
	beforeEach(async () => {
		await initWasm();
	});

	it('should add 1 + 2 to equal 3', async () => {
		const result = createSVGComposition({ width: 100, height: 100 });
		expect(result).toBeDefined();
	});
});
