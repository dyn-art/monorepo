import { createServer as createHttpServer } from 'node:http';
import { appConfig } from '@/environment';
import { logger } from '@/logger';

async function initAsyncDependencies(): Promise<void> {
	logger.info('Initializing async modules...');

	// Init DB connection
	// TODO:

	logger.info('Initialized async modules.');
}

// Initialize the server
(async () => {
	// Load the async dependencies before starting the Express app
	await initAsyncDependencies();

	// Import the Express app module after async dependencies are initialized
	// to ensure that the app only starts handling requests after all dependencies are ready
	const { app } = await import('./app');
	app.set('port', appConfig.port);

	// Create the HTTP server with the Express app as a request listener
	const httpServer = createHttpServer(app);
	httpServer.listen(appConfig.port);

	// Set up server event listeners
	httpServer.on('error', (error) => {
		logger.error(`Error occurred in http server: ${error.message}`);
	});
	httpServer.on('listening', () => {
		logger.info(`Server running on Port: ${appConfig.port}`);
	});
})().catch(() => {
	// do nothing
});
