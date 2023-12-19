import type { TAppMessageEvent } from '@dyn/figma-handler/app';

import type { EAppRoutes } from './app';

export interface TOnUIRouteChangeEvent extends TAppMessageEvent {
	key: 'on-ui-route-change';
	args: {
		activeRoute: EAppRoutes;
	};
}

export interface TIntermediateFormatExportEvent extends TAppMessageEvent {
	key: 'intermediate-format-export';
	args: {
		selectedElements: Pick<FrameNode | ComponentNode | InstanceNode, 'name' | 'id'>[];
	};
}

export type TAppMessageEvents = TOnUIRouteChangeEvent | TIntermediateFormatExportEvent;
