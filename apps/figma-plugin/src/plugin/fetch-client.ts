import { createGoogleClient } from '@dyn/google-client';

import { googleConfig } from './environment';

export const googleClient = createGoogleClient({
	apiKey: googleConfig.fontApiToken
});
