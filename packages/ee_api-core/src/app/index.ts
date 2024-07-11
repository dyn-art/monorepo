import { Hono } from 'hono';

import { errorHandler, invalidPathHandler } from './handlers';
import { router } from './router';

import './routes';

export function createApp(app: Hono = new Hono()): Hono {
	app.onError(errorHandler);
	app.notFound(invalidPathHandler);
	app.route('/', router);

	return app;
}
