import { pickProperties } from '@dyn/utils';

import type { TPluginCallbackRegistration, TPluginHandler } from '../../types';

export default {
	type: 'selectionchange',
	key: 'selection-change',
	callback: async (instance: TPluginHandler) => {
		const selection = figma.currentPage.selection;
		const selectedFrames = selection.filter(
			(node) => node.type === 'FRAME' || node.type === 'COMPONENT'
		);

		console.log('Plugin: onSelectionChange', { selection, selectedFrames }); // TODO: REMOVE

		// Post on select frame to app part
		if (selectedFrames.length > 0) {
			instance.post('on-select-frame', {
				selected: selectedFrames.map((frame) => pickProperties(frame, ['name', 'id']))
			});
		} else {
			instance.post('on-select-frame', { selected: [] });
		}

		// Post on select node to app part
		instance.post('on-select-node', {
			selected: selection.map((node) => pickProperties(node, ['name', 'id']))
		});
	}
} as TPluginCallbackRegistration;
