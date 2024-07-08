import { openApiRouter } from './router';

openApiRouter.get('/v1/health', {
	handler: (_req, res) => {
		res.send({
			message: 'Up and running',
			status: 'Up'
		});
	}
});
