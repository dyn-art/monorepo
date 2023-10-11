import { beforeEach, describe, expect, it } from 'vitest';

import { editorFactory, initWasm } from '..';

describe('dtom', () => {
	beforeEach(async () => {
		await initWasm();
	});

	it('should add 1 + 2 to equal 3', async () => {
		const result = editorFactory();
		expect(result).toBeDefined();
	});
});
