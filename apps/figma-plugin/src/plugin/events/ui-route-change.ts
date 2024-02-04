import type { TCustomPluginCallbackRegistration } from '../../types';
import { ACTIVE_APP_ROUTE } from '../core/ui';

export const uiRouteChange: TCustomPluginCallbackRegistration = {
	type: 'app.message',
	key: 'on-ui-route-change',
	callback: async (instance, args) => {
		ACTIVE_APP_ROUTE.set(args.activeRoute);
	}
};
