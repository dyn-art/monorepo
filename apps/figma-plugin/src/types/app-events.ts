import type { TFromAppMessageEvent } from 'figma-connect/app';

import type { EAppRoutes } from './app';

export interface TOnUIRouteChangeEvent extends TFromAppMessageEvent {
	key: 'on-ui-route-change';
	args: {
		activeRoute: EAppRoutes;
	};
}

export interface TIntermediateFormatExportEvent extends TFromAppMessageEvent {
	key: 'intermediate-format-export';
	args: {
		selectedElements: Pick<FrameNode | ComponentNode | InstanceNode, 'name' | 'id'>[];
	};
}

export type TFromAppMessageEvents = TOnUIRouteChangeEvent | TIntermediateFormatExportEvent;
