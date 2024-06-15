import { app } from './app';

const server = Bun.serve({
	fetch: app.fetch,
	port: process.env.APP_PORT || 3009
});

console.log(`Running on port: ${server.port}`);
