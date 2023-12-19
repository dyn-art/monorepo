import { Transformer } from '@dyn/figma-to-dtif';

import type { TPluginCallbackRegistration } from '../../types';

export default {
	type: 'app.message',
	key: 'intermediate-format-export',
	callback: async (instance, args) => {
		console.log('intermediate-format-export', args);

		// Filter out unsupported nodes
		const supportedNodes = args.selectedElements
			.map((element) => instance.figma.currentPage.findOne((node) => node.id === element.id))
			.filter(
				(node) => node != null && ['FRAME', 'COMPONENT', 'INSTANCE'].includes(node.type)
			) as FrameNode[];

		// Process nodes
		for (const node of supportedNodes) {
			const transformer = new Transformer(node);
			const result = await transformer.transform({
				font: {
					exportOptions: { inline: true },
					resolveFontContent: async () => {
						// TODO
						return null as any;
					}
				},
				paint: {
					gradientExportOptions: { inline: true },
					imageExportOptions: { inline: true }
				}
				// node: {
				// 	includeInvisible: false
				// }
			});
			console.log({ transformer, result });
		}
	}
} as TPluginCallbackRegistration;
