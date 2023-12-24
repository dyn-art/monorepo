import { createFetchClient } from './create-fetch-client';
import { withApi } from './features';
import type { TFetchClient } from './types';

export function createApiFetchClient(): TFetchClient<['api']> {
	return withApi(createFetchClient());
}
