import { describe, it } from 'vitest';

import type { paths } from '../__tests__/resources/mock-openapi-types';
import { createFetchClient } from '../create-fetch-client';
import { withOpenApi } from './with-openapi';

describe('withOpenApi function tests', () => {
	it('should have correct types', async () => {
		const baseFetchClient = createFetchClient();
		const fetchClient = withOpenApi<paths>(baseFetchClient);
		fetchClient.get('/v1/media/pre-signed-download-url/{key}', {
			pathParams: {
				key: 'jeff'
			}
		});
	});
});
