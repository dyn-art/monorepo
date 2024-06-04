import { createGoogleClient } from 'google-webfonts-client';

import { googleConfig } from './environment';

export const googleClient = createGoogleClient({
	apiKey: googleConfig.fontApiToken
});
