import type { TPluginCallbackRegistration } from '@dyn/figma-handler/app';

import { onPropertyChange } from './events';

export default {
	type: 'documentchange',
	key: 'document-change-event',
	callback: (instance, args) => {
		for (const documentChange of args.documentChanges) {
			switch (documentChange.type) {
				case 'PROPERTY_CHANGE':
					onPropertyChange(instance, documentChange);
					break;
				default:
				// do nothing
			}
		}
	}
} as TPluginCallbackRegistration;
