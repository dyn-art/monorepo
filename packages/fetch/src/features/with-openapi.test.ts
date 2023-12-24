import { describe, it } from 'vitest';

import { createFetchClient } from '../create-fetch-client';
import { withOpenApi } from './with-open-api';

describe('withOpenApi function tests', () => {
	it('should have correct types', async () => {
		const fetchClient = withOpenApi(createFetchClient());
	});
});
