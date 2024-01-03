import { describe, it } from 'vitest';

import type { paths } from './__tests__/resources/mock-openapi-types';
import { createFetchExpress } from './create-fetch-express';

describe('createFetchExpress function tests', () => {
	it('should have correct types', async () => {
		const baseFetchClient = createFetchExpress<paths>();

		baseFetchClient.get('/v1/ping', null, async (req, res, next) => {
			// TODO
		});

		baseFetchClient.get('/v1/media/pre-signed-download-url/{key}', null, async (req, res, next) => {
			const params = req.params;
			// TODO
		});

		// TODO
	});
});
