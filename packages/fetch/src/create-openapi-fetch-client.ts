import { createFetchClient } from './create-fetch-client';
import { withOpenApi } from './features';
import type { TFetchClient } from './types';

export function createOpenApiFetchClient(): TFetchClient<['openapi']> {
	return withOpenApi(createFetchClient());
}
