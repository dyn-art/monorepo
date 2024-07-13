import { logger } from '@/logger';

// Only load .env in development
const nodeEnv = process.env.NODE_ENV ?? 'local';
if (nodeEnv === 'local') {
	// eslint-disable-next-line @typescript-eslint/no-var-requires -- Non async dynamic import
	require('dotenv').config({ path: `.env.${nodeEnv}` });
	logger.info(`Loaded dotenv from '.env.${nodeEnv}'.`);
}

export * from './app.config';
export * from './db.config';
