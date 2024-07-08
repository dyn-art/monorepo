import { openApiRouter } from './router';

openApiRouter.get('/v1/health', {
	handler: async (req, res) => {
		res.status(200).send({});
	}
});
