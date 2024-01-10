import { registerPluginEventCallback } from '../../plugin-handler';
import { onPropertyChange } from './events';

registerPluginEventCallback({
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
});
