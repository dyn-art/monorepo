import { Router } from 'express';
import { z } from 'zod';
import { createOpenApiRouter } from '@dyn/openapi-router';
import type { paths } from '@dyn/types/core';

export const router: Router = Router();

const openApiRouter = createOpenApiRouter<paths>(router);

openApiRouter.get('/v1/ping', {}, async (req, res) => {
	res.status(200).send(true);
});

openApiRouter.get(
	'/v1/hello',
	{
		querySchema: {
			name: z.string()
		}
	},
	async (req, res) => {
		const { name } = req.query;

		res.status(200).send(`Hello ${name}`);
	}
);
