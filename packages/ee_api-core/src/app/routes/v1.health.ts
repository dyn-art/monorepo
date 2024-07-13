import { openApiRouter } from '../router';

openApiRouter.get('/v1/health', {
	handler: (c) => {
		return c.json({
			message: 'App is up and running',
			status: 'Up'
		});
	}
});
