import type { TAppCallbackRegistration, TAppMessageEvent, TPluginMessageEvent } from '../types';
import { AppCallback } from './AppCallback';

export class FigmaAppHandler<
	GPluginMessageEvent extends TPluginMessageEvent = TPluginMessageEvent,
	GAppMessageEvent extends TAppMessageEvent = TAppMessageEvent
> {
	private readonly parent: Window;

	constructor(parentInstance: Window, options: TFigmaAppHandlerOptions<GPluginMessageEvent> = {}) {
		const { events = [] } = options;
		this.parent = parentInstance;
		this.register(events);
	}

	public register(
		registrations:
			| TAppCallbackRegistration<GPluginMessageEvent>
			| TAppCallbackRegistration<GPluginMessageEvent>[]
	): void {
		const callbacks = Array.isArray(registrations)
			? registrations.map((r) => new AppCallback(r))
			: [new AppCallback(registrations)];

		this.registerCallbacks(callbacks);
	}

	public post<GKey extends GAppMessageEvent['key']>(
		key: GKey,
		args: Extract<GAppMessageEvent, { key: GKey }>['args']
	): void {
		this.parent.postMessage({ pluginMessage: { key, args } }, '*');
	}

	// =========================================================================
	// Helper
	// =========================================================================

	private registerCallbacks(callbacks: AppCallback<GPluginMessageEvent>[]): void {
		callbacks.forEach((callback) => {
			this.registerCallback(callback);
		});
	}

	private registerCallback(callback: AppCallback<GPluginMessageEvent>): void {
		let type: string = callback.type;
		const typeParts = type.split('.');
		if (typeParts.length === 2) {
			type = typeParts[1] as unknown as string;
		}

		// Note: Using global 'addEventListener' to avoid cross-origin frame access errors.
		// Attempting to call 'parent.x' results in a DOMException for cross-origin frame access.
		addEventListener(type as any, (...args) => {
			if (callback.shouldCall()) {
				this.onEvent(callback, args).catch(() => {
					// do nothing
				});
			}
		});
	}

	private async onEvent(callback: AppCallback<GPluginMessageEvent>, args: any[]): Promise<void> {
		if (callback.type === 'plugin.message') {
			const data = args[0]?.data;
			const pluginMessage = data?.pluginMessage;
			if (
				pluginMessage != null &&
				pluginMessage?.key === callback.key &&
				typeof pluginMessage?.args === 'object'
			) {
				await callback.callback(this, {
					pluginId: data?.pluginId,
					...pluginMessage.args
				});
			}
		} else {
			await callback.callback(this, ...args);
		}
	}
}

export interface TFigmaAppHandlerOptions<GPluginMessageEvent extends TPluginMessageEvent> {
	events?: TAppCallbackRegistration<GPluginMessageEvent>[];
}
