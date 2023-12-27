const baseUrl = process.env.GOOGLE_BASE_URL ?? 'https://www.googleapis.com/webfonts/v1';
const apiKey = process.env.GOOGLE_API_KEY ?? 'not-set';

export const googleConfig = {
	baseUrl,
	auth: {
		apiKey
	}
};
