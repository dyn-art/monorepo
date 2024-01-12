import { ACTIVE_APP_ROUTE } from '../core/ui';
import { registerPluginEventCallback } from '../plugin-handler';

registerPluginEventCallback({
	type: 'app.message',
	key: 'on-ui-route-change',
	callback: async (instance, args) => {
		ACTIVE_APP_ROUTE.set(args.activeRoute);
	}
});
