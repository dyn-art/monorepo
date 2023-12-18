import { Transformer } from '@dyn/figma-to-dtif';
import { pickProperties } from '@dyn/utils';

import { EAppRoutes, type TPluginCallbackRegistration, type TPluginHandler } from '../../types';
import { ACTIVE_APP_ROUTE, SELECTED_NODE_IDS } from '../core/ui';
import { getObjectPropertyKeys } from '../core/utils';

export default {
	type: 'selectionchange',
	key: 'selection-change',
	callback: async (instance: TPluginHandler) => {
		const selection = figma.currentPage.selection;
		const selectedFrames: FrameNode[] = selection.filter(
			(node) => node.type === 'FRAME' || node.type === 'COMPONENT'
		) as unknown as FrameNode[];

		console.log('Plugin: onSelectionChange', { selection, selectedFrames }); // TODO: REMOVE

		// Update state
		SELECTED_NODE_IDS.set(selection.map((node) => node.id));

		// Post on select node to app part
		instance.post('on-select-node', {
			selected: selection.map((node) => pickProperties(node, ['name', 'id']))
		});

		// Post on select frame to app part
		if (ACTIVE_APP_ROUTE.get()?.toString() === `${EAppRoutes.HOME}${EAppRoutes.HOME__TO_DTIF}`) {
			if (selectedFrames.length > 0) {
				instance.post('on-select-frame', {
					selected: selectedFrames.map((frame) => pickProperties(frame, ['name', 'id']))
				});
				for (const selectedFrame of selectedFrames) {
					const transformer = new Transformer(selectedFrame);
					await transformer.transform();
					console.log({ transformer });
				}
			} else {
				instance.post('on-select-frame', { selected: [] });
			}
		}

		// Post on select node properties to app part
		if (
			ACTIVE_APP_ROUTE.get()?.toString() === `${EAppRoutes.HOME}${EAppRoutes.HOME__NODE_INSPECTOR}`
		) {
			instance.post('on-select-node-properties', {
				selected: selection.map((node) =>
					pickProperties(node, getObjectPropertyKeys(node) as any)
				) as SceneNode[]
			});
		}
	}
} as TPluginCallbackRegistration;
