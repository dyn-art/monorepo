import { describe, it } from 'vitest';

import { createFetchClient } from '../create-fetch-client';
import { withApi } from './with-api';

describe('withApi function tests', () => {
	it('should have correct types', async () => {
		const fetchClient = withApi(createFetchClient());
	});
});
