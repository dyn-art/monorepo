import { app } from './app';

Bun.serve({
	fetch: app.fetch,
	port: process.env.APP_PORT || 3009
});
