import { assertValue } from '@ibg/utils';

export const appConfig = {
	url: assertValue(process.env.NEXT_PUBLIC_URL, 'NEXT_PUBLIC_URL not set!'),
	meta: {
		title: {
			default: 'dyn.art | App Platform',
			template: (title: string) => `${title} | dyn.art`
		},
		description: 'Leave the repetitive design to us'
	}
};
