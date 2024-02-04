import type { Result, TFetchClient, TOpenApiFetchResponse } from '@dyn/fetch-client';

import type { paths } from './gen/v1';

export * from '@dyn/fetch-client';
export * from './create-google-client';

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
			): Promise<Result<string | null, Error>>;
			downloadFontFile(
				familiy: Omit<paths['/webfonts']['get']['parameters']['query'], 'key'>['family'],
				options: {
					fontWeight?: number;
					fontStyle?: 'italic' | 'regular';
					capability?: Omit<paths['/webfonts']['get']['parameters']['query'], 'key'>['capability'];
				}
			): Promise<Result<Uint8Array | null, Error>>;
		};
	}
}
