import { isFigmaRemovedNode } from '@dyn/figma-to-dtif';
import { pickProperties } from '@dyn/utils';

import { EAppRoutes, type TPluginHandler } from '../../../../types';
import { ACTIVE_APP_ROUTE, SELECTED_NODE_IDS } from '../../../core/ui';
import { getObjectPropertyKeys } from '../../../core/utils';

export function onPropertyChange(instance: TPluginHandler, event: PropertyChange): void {
	if (
		ACTIVE_APP_ROUTE.get()?.toString() === `${EAppRoutes.HOME}${EAppRoutes.HOME__NODE_INSPECTOR}`
	) {
		const node = event.node;

		// Post on change selected node properties UI event
		if (SELECTED_NODE_IDS.get().includes(node.id) && !isFigmaRemovedNode(node)) {
			instance.post('on-change-selected-node-properties', {
				changed: pickProperties(node, getObjectPropertyKeys(node) as any) as SceneNode
			});
		}
	}
}
