import { assertValue } from '../utils';

export const appConfnig = {
	url: assertValue(process.env.NEXT_PUBLIC_URL, 'NEXT_PUBLIC_URL not set!'),
	meta: {
		title: 'todo',
		description: 'todo'
	}
};
