import type { TFetchClient, TOpenApiFetchResponse } from '@dyn/fetch-client';

import type { paths } from './gen/v1';

declare module '@dyn/fetch-client' {
	interface TThirdPartyFeatures {
		google: {
			rawFetchClient: TFetchClient<['base', 'api']>;
			getWebFonts(
				options?: Omit<paths['/webfonts']['get']['parameters']['query'], 'key'>
			): Promise<TOpenApiFetchResponse<paths['/webfonts']['get'], 'json'>>;
			getFontFileUrl(
				familiy: Omit<paths['/webfonts']['get']['parameters']['query'], 'key'>['family'],
				options: {
					fontWeight?: number;
					fontStyle?: 'italic' | 'regular';
					capability?: Omit<paths['/webfonts']['get']['parameters']['query'], 'key'>['capability'];
				}
			): Promise<string | null>;
			downloadFontFile(
				familiy: Omit<paths['/webfonts']['get']['parameters']['query'], 'key'>['family'],
				options: {
					fontWeight?: number;
					fontStyle?: 'italic' | 'regular';
					capability?: Omit<paths['/webfonts']['get']['parameters']['query'], 'key'>['capability'];
				}
			): Promise<Uint8Array | null>;
		};
	}
}
