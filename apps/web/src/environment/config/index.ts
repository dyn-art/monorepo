import { logger } from '@/logger';

import { appConfig } from './app';
import { marketingConfig } from './marketing';

export * from './app';
export * from './marketing';

logger.info('✅ Loaded configuration', { appConfnig: appConfig, marketingConfig });
