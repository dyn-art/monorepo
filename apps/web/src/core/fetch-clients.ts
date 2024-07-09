import { createOpenApiFetchClient } from 'feature-fetch';
import { type paths } from '@dyn/types/core';

export const coreClient = createOpenApiFetchClient<paths>({
	prefixUrl: 'http://localhost:9000'
});
