import { beforeEach, describe, expect, it } from 'vitest';

import { greetRust, initWasm } from '..';

describe('dtom', () => {
	beforeEach(async () => {
		await initWasm();
	});

	it('should add 1 + 2 to equal 3', async () => {
		const result = greetRust();
		console.log(result);
		expect(3).toBe(3);
	});
});
