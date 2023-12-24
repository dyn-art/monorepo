import type { TFetchClient } from './types';

export function createFetchClient(): TFetchClient<['base']> {
	return {
		_: null,
		_config: { prefixUrl: '' },
		_baseFetch(path, method, options) {
			return null as any;
		}
	};
}
