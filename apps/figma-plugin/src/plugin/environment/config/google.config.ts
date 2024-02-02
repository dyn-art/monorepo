import { assertValue } from '../utils';

export const googleConfig = {
	fontApiToken: assertValue(
		process.env.GOOGLE_FONT_API_TOKEN,
		'Missing environment variable: GOOGLE_FONT_API_TOKEN'
	)
};
