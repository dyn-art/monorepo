import type { TCustomPluginCallbackRegistration } from '../../../types';
import { onPropertyChange } from './events';

export const documentChangeEvent: TCustomPluginCallbackRegistration = {
	type: 'documentchange',
	key: 'document-change-event',
	callback: async (instance, args) => {
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
};
