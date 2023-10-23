import { beforeEach, describe, expect, it } from 'vitest';

import { createSVGCanvas } from '../canvas';
import { initWasm } from '../wasm';

describe('dtom', () => {
	beforeEach(async () => {
		await initWasm();
	});

	it('should add 1 + 2 to equal 3', async () => {
		const result = createSVGCanvas({ width: 100, height: 100 });
		expect(result).toBeDefined();
	});
});
