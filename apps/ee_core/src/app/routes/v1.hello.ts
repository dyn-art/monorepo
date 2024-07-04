import { z } from 'zod';

import { openApiRouter } from './router';

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
