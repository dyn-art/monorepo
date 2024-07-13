import { createLogger, LOG_LEVEL, withPrefix } from 'feature-logger';

export const logger = withPrefix(
	createLogger({
		level: LOG_LEVEL.INFO
	}),
	'@dyn/core'
);
