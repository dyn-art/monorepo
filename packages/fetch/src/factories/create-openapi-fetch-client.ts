import { withOpenApi } from '../features';
import type { TFetchClient } from '../types';
import { createFetchClient } from './create-fetch-client';

export function createOpenApiFetchClient<GPaths extends {} = {}>(): TFetchClient<
	['base', 'openapi'],
	GPaths
> {
	return withOpenApi(createFetchClient());
}
