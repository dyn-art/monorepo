import { NodeException, Transformer, type COMP } from '@dyn/figma-to-dtif';
import { extractErrorData, notEmpty, sleep } from '@dyn/utils';

import type { TCustomPluginCallbackRegistration, TPluginHandler } from '../../types';
import { googleClient } from '../fetch-client';

export const intermediateFormatExport: TCustomPluginCallbackRegistration = {
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
};

async function processNode(node: FrameNode, instance: TPluginHandler): Promise<void> {
	instance.post('on-transform-status-update', { type: 'Start' });
	const transformer = new Transformer(node, {
		onTransformStatusUpdate: (status) => {
			instance.post('on-transform-status-update', { type: 'Transform', status });
		}
	});

	try {
		const result = await transformer.transform({
			asset: {
				font: {
					export: { mode: 'Inline' },
					resolveFontContent: async (fontInfo) => {
						const urlResponse = await googleClient.getFontFileUrl(
							typeof fontInfo.family === 'string' ? fontInfo.family : fontInfo.family.Named,
							{
								fontWeight: fontInfo.variant.weight,
								fontStyle: fontInfo.variant.style === 'Italic' ? 'italic' : 'regular'
							}
						);
						const url = urlResponse.unwrap();
						if (url == null) {
							return null;
						}

						return {
							type: 'Url',
							url,
							contentType: { mimeType: 'font/ttf' }
						};
					}
				},
				image: { export: { format: 'PNG', mode: 'Inline' } }
			},
			node: {
				includeInvisible: false,
				shouldExportFrame: { format: 'PNG' }
			}
		});
		await handleSuccess(result, node, instance);
	} catch (error) {
		handleError(error, node, instance);
	}
	instance.post('on-transform-status-update', { type: 'End' });
}

async function handleSuccess(
	result: COMP.DtifComposition,
	node: SceneNode,
	instance: TPluginHandler
): Promise<void> {
	instance.post('on-transform-status-update', { type: 'Transmit' });
	await sleep(100);
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
		const errorCausingNodes: SceneNode[] = error.nodeIds
			.map((nodeId) => figma.getNodeById(nodeId) as SceneNode)
			.filter(notEmpty);
		if (errorCausingNodes.length > 0) {
			figma.currentPage.selection = errorCausingNodes;
		}
	}
}
