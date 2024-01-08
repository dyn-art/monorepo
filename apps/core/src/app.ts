import express, { type Express } from 'express';

import { router } from './routes';

export const app: Express = express();

// Enable 'trust proxy' to reveal the real client IP address for rate limiting
app.set('trust proxy', 1);

// Add middleware to parse JSON request bodies
app.use(express.json());

// Register application routes
app.use('/', router);