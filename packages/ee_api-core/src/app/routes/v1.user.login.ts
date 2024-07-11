import * as v from 'valibot';
import { vValidator } from 'validation-adapters/valibot';

import { openApiRouter } from '../router';

openApiRouter.get('/v1/user/login', {
	queryValidator: vValidator(
		v.object({
			email: v.pipe(v.string(), v.nonEmpty(), v.email()),
			password: v.pipe(v.string(), v.nonEmpty())
		})
	),
	handler: (c) => {
		// TODO
		return c.text('token');
	}
});
