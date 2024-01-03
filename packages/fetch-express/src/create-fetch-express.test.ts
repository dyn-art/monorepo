import { describe, it } from 'vitest';
import { z } from 'zod';

import type { paths } from './__tests__/resources/mock-openapi-types';
import { createFetchExpress } from './create-fetch-express';

describe('createFetchExpress function tests', () => {
	it('should have correct types', async () => {
		const baseFetchClient = createFetchExpress<paths>();

		baseFetchClient.get('/v1/ping', null, async (req, res, next) => {
			// TODO

			res.status(200).send(false);
		});

		baseFetchClient.get('/v1/auth/etsy/oauth/redirect', {}, (req, res, next) => {
			const query = req.query;
		});

		baseFetchClient.get('/v1/media/pre-signed-download-url/{key}', null, async (req, res, next) => {
			const params = req.params;
			// TODO
		});

		const zod = z.object({
			jeff: z.string()
		});

		// TODO
	});
});
