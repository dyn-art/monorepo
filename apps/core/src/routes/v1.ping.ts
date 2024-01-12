import { openApiRouter } from './router';

openApiRouter.get('/v1/ping', {}, async (req, res) => {
	res.status(200).send(true);
});
