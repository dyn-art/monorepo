import { NodeException, Transformer, type COMP } from '@dyn/figma-to-dtif';
import { extractErrorData } from '@dyn/utils';

import { type TPluginHandler } from '../../types';
import { registerPluginEventCallback } from '../plugin-handler';

registerPluginEventCallback({
	type: 'app.message',
	key: 'intermediate-format-export',
	callback: async (instance: TPluginHandler, args) => {
		console.log('intermediate-format-export', args);

		// Filter out unsupported nodes
		const supportedNodes = args.selectedElements
			.map((element) => instance.figma.currentPage.findOne((node) => node.id === element.id))
			.filter(
				(node) => node != null && ['FRAME', 'COMPONENT', 'INSTANCE'].includes(node.type)
			) as FrameNode[];

		// Process nodes
		for (const node of supportedNodes) {
			await processNode(node, instance);
		}
	}
});

async function processNode(node: FrameNode, instance: TPluginHandler): Promise<void> {
	const transformer = new Transformer(node, {
		onTransformStatusUpdate: (status) => {
			instance.post('on-transform-status-update', { status });
		}
	});

	try {
		const result = await transformer.transform({
			font: {
				export: { mode: 'Inline' },
				resolveFontContent: async () => {
					// TODO
					return null as any;
				}
			},
			paint: {
				gradient: { inline: true },
				image: { export: { format: 'PNG', mode: 'Inline' } }
			}
			// node: {
			// 	includeInvisible: false
			// }
		});
		handleSuccess(result, node, instance);
	} catch (error) {
		handleError(error, node, instance);
	}
}

function handleSuccess(
	result: COMP.DTIFComposition,
	node: SceneNode,
	instance: TPluginHandler
): void {
	instance.post('intermediate-format-export-result', {
		type: 'success',
		content: result
	});
	const successMessage = `Successfully exported node '${node.name}' :)`;
	figma.notify(successMessage);
}

function handleError(error: unknown, node: SceneNode, instance: TPluginHandler): void {
	const { message } = extractErrorData(error);
	instance.post('intermediate-format-export-result', {
		type: 'error',
		message
	});
	const figmaMessage = `Error exporting node '${node.name}': ${message}`;
	figma.notify(figmaMessage, {
		error: true
	});
	if (error instanceof NodeException) {
		const errorCausingNode = figma.getNodeById(error.nodeId);
		if (errorCausingNode != null) {
			figma.currentPage.selection = [errorCausingNode as SceneNode];
		}
	}
}
