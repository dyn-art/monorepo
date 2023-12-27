import { withApi } from '../features';
import type { TFetchClient } from '../types';
import { createFetchClient } from './create-fetch-client';

export function createApiFetchClient(): TFetchClient<['base', 'api']> {
	return withApi(createFetchClient());
}
