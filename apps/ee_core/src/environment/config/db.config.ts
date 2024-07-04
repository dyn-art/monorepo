import { assertValue } from '../utils';

export const dbConfig = {
	url: assertValue(process.env.DATABASE_URL, 'Environment variable "DATABASE_URL" not set!')
};
