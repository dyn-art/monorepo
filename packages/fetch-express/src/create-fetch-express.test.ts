import { describe, it } from 'vitest';
import { z } from 'zod';

import type { paths } from './__tests__/resources/mock-openapi-types';
import { createFetchExpress } from './create-fetch-express';

describe('createFetchExpress function tests', () => {
	it('should have correct types', async () => {
		const baseFetchClient = createFetchExpress<paths>();

		baseFetchClient.get(
			'/v1/media/pre-signed-upload-url',
			{
				querySchema: {
					key: z.string().optional(),
					content_type: z.string().optional(),
					overwrite: z.boolean().optional(),
					scope: z.string().optional()
				},
				pathSchema: {
					key: z.string()
				}
			},
			async (req) => {
				const query = req.query;
			}
		);

		baseFetchClient.post(
			'/v1/ping',
			{
				bodySchema: {
					hello: z.string(),
					jeff: z
						.object({
							num1: z.string(),
							num2: z.number(),
							moreNested: z.object({
								hello: z.string()
							})
						})
						.optional()
				},
				pathSchema: {
					shop_id: z.number()
				},
				querySchema: {
					test123: z.number()
				}
			},
			async (req, res, next) => {
				const params = req.params;
				// TODO
			}
		);

		const zod = z.object({
			jeff: z.string()
		});

		// TODO
	});
});
