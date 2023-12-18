import type { TPluginCallbackRegistration } from '../../types';
import { ACTIVE_APP_ROUTE } from '../core/ui';

export default {
	type: 'app.message',
	key: 'on-ui-route-change',
	callback: (instance, args) => {
		ACTIVE_APP_ROUTE.set(args.activeRoute);
	}
} as TPluginCallbackRegistration;
