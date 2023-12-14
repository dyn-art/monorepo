import type { TAppMessageEvent } from '@dyn/figma-handler/app';

import type { EAppRoutes } from './ui';

export interface TOnUIRouteChangeEvent extends TAppMessageEvent {
	key: 'on-ui-route-change';
	args: {
		activeRoute: EAppRoutes;
	};
}

export type TAppMessageEvents = TOnUIRouteChangeEvent;
