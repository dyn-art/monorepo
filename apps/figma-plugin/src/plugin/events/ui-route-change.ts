import type { TPluginCallbackRegistration } from '../../types';

export default {
	type: 'app.message',
	key: 'on-ui-route-change',
	callback: (instance, args) => {
		console.log('Plugin: onUIRouteChange', args); // TODO: REMOVE
	}
} as TPluginCallbackRegistration;
