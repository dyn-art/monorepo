import { openApiRouter } from './router';

openApiRouter.get('/v1/health', {
	handler: (_req, res) => {
		res.send({
			message: 'App is up and running',
			status: 'Up'
		});
	}
});
