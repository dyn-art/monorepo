import { shortId } from '@dyn/utils';

import type { TAppEventRegistration, TPluginMessageEvent } from '../types';

export class AppCallback<
	GPluginMessageEvent extends TPluginMessageEvent,
	GAppEventRegistration extends
		TAppEventRegistration<GPluginMessageEvent> = TAppEventRegistration<GPluginMessageEvent>
> {
	public readonly key: string;
	public readonly type: GAppEventRegistration['type'];
	public readonly callback: GAppEventRegistration['callback'];
	public readonly once: boolean;

	private _wasCalled = false;

	constructor(registration: GAppEventRegistration) {
		this.key = registration.key != null ? `${shortId('xxxx')}_${registration.key}` : shortId();
		this.type = registration.type;
		this.callback = registration.callback;
		this.once = registration.once ?? false;
	}

	public shouldCall(): boolean {
		if (this.once) {
			if (!this._wasCalled) {
				this._wasCalled = true;
				return true;
			}
			return false;
		}
		return true;
	}
}
