import * as v from 'valibot';
import { valibotAdapter } from 'validation-adapters/valibot';

import { openApiRouter } from './router';

openApiRouter.get('/v1/user/login', {
	queryAdapter: valibotAdapter(
		v.object({
			email: v.pipe(v.string(), v.nonEmpty(), v.email()),
			password: v.pipe(v.string(), v.nonEmpty())
		})
	),
	handler: (req, res) => {
		// TODO
	}
});
