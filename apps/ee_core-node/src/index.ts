import { serve } from '@hono/node-server';

(async () => {
	// Only load .env in development and before loading the app
	const nodeEnv = process.env.NODE_ENV ?? 'local';
	if (nodeEnv === 'local') {
		const dotenv = await import('dotenv');
		dotenv.config({ path: `.env.${nodeEnv}` });
		console.log(`Loaded dotenv from '.env.${nodeEnv}'.`);
	}

	const { createApp, logger } = await import('@dyn/api-core');

	const port = 8787;
	const app = createApp();

	logger.info(`Server is running on port ${port.toString()}`);

	serve({
		fetch: app.fetch,
		port
	});
})().catch((e: unknown) => {
	console.error('Failed to start server by exception: ', e);
});
