import { appConfig } from './app';
import { marketingConfig } from './marketing';

export * from './app';
export * from './marketing';

console.log('✅ Loaded configuration', { appConfnig: appConfig, marketingConfig });
