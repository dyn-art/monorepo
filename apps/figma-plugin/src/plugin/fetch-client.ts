import { createGoogleWebfontsClient } from 'google-webfonts-client';

import { googleConfig } from './environment';

export const googleWebfontsClient = createGoogleWebfontsClient({
	apiKey: googleConfig.fontApiToken
});
