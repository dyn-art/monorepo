import { LOG_LEVEL, Logger } from '@dyn/logger';

export const logger = new Logger({
	prefix: '@dyn/gateway',
	level: LOG_LEVEL.INFO
});
