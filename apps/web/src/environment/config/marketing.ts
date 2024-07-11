export const marketingConfig = {
	meta: {
		title: {
			default: 'dyn.art | Leave the repetitive design to us',
			template: (title: string) => `${title} | dyn.art`
		},
		description: 'Leave the repetitive design to us'
	},
	navbar: {
		links: [
			{
				title: 'Pricing',
				path: '/pricing'
			},
			{
				title: 'Updates',
				path: '/updates'
			},
			{
				title: 'Demos',
				path: '/demos'
			},
			{
				title: 'Docs',
				path: '/docs'
			}
		]
	}
};
