import { describe, it } from 'vitest';

import type { paths } from '../__tests__/resources/mock-openapi-types';
import { createFetchClient } from '../create-fetch-client';
import { withOpenApi } from './with-openapi';

describe('withOpenApi function tests', () => {
	it('should have correct types', async () => {
		const baseFetchClient = createFetchClient();
		const fetchClient = withOpenApi<paths>(baseFetchClient);
		const response = await fetchClient.get('/v1/media/pre-signed-download-url/{key}', {
			pathParams: {
				key: ''
			}
		});

		const response2 = await fetchClient.post(
			'/v1/ping',
			{ jeff: 1, hello: 'world' },
			{
				queryParams: {
					test123: 1
				},
				pathParams: {
					shop_id: 1
				}
			}
		);

		const jeff = response2.unwrap();

		fetchClient.get('/v1/ping', {
			pathParams: { test: '' }
		});

		// fetchClient.post('/v1/ping', )
	});
});
