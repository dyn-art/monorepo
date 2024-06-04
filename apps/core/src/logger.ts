import { LOG_LEVEL, Logger } from 'feature-logger';

export const logger = new Logger({
	prefix: '@dyn/core',
	level: LOG_LEVEL.INFO
});
