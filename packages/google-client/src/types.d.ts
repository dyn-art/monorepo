import type { TOpenApiFetchResponse } from '@dyn/fetch-client';

import type { paths } from './gen/v1';

declare module '@dyn/fetch-client' {
	interface TThirdPartyFeatures {
		google: {
			getWebFonts(
				options?: Omit<paths['/webfonts']['get']['parameters']['query'], 'key'>
			): Promise<TOpenApiFetchResponse<paths['/webfonts']['get'], 'json'>>;
		};
	}
}
