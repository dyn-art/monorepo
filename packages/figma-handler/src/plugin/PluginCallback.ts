import { shortId } from '@dyn/utils';

import type { TAppMessageEvent, TPluginEventRegistration } from '../types';

export class PluginCallback<
	GAppMessageEvent extends TAppMessageEvent,
	GPluginEventRegistration extends
		TPluginEventRegistration<GAppMessageEvent> = TPluginEventRegistration<GAppMessageEvent>
> {
	public readonly key: string;
	public readonly type: GPluginEventRegistration['type'];
	public readonly callback: GPluginEventRegistration['callback'];
	public readonly once: boolean;

	constructor(registration: GPluginEventRegistration) {
		this.key = registration.key != null ? `${shortId('xxxx')}_${registration.key}` : shortId();
		this.type = registration.type;
		this.callback = registration.callback;
		this.once = registration.once ?? false;
	}
}
