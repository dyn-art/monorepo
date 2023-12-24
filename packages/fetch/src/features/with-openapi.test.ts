import { describe, it } from 'vitest';

import { createFetchClient } from '../create-fetch-client';
import { withOpenApi } from './with-openapi';

describe('withOpenApi function tests', () => {
	it('should have correct types', async () => {
		const baseFetchClient = createFetchClient();
		const fetchClient = withOpenApi(baseFetchClient as any);
	});
});
