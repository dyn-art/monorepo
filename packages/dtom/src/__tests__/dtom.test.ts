import { expect, test } from 'vitest';

import { greetRust } from '..';

test('adds 1 + 2 to equal 3', async () => {
	expect(await greetRust()).toBe(3);
});
