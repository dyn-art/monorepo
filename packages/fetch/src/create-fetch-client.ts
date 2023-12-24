import type { TFetchClient } from './types';

export function createFetchClient(): TFetchClient<[]> {
	return {
		_config: { prefixUrl: '' },
		_baseFetch(path, method, options) {
			return null as any;
		}
	};
}
