import type { TAppMessageEvent, TPluginCallbackRegistration, TPluginMessageEvent } from '../types';
import { PluginCallback } from './PluginCallback';

export class FigmaPluginHandler<
	GAppMessageEvent extends TAppMessageEvent = TAppMessageEvent,
	GPluginMessageEvent extends TPluginMessageEvent = TPluginMessageEvent
> {
	public readonly figma: typeof figma;

	constructor(
		figmaInstance: typeof figma,
		options: TFigmaPluginHandlerOptions<GAppMessageEvent> = {}
	) {
		const { events = [] } = options;
		this.figma = figmaInstance;
		this.register(events);
	}

	public register(
		registrations:
			| TPluginCallbackRegistration<GAppMessageEvent>
			| TPluginCallbackRegistration<GAppMessageEvent>[]
	): void {
		const callbacks = Array.isArray(registrations)
			? registrations.map((r) => new PluginCallback(r))
			: [new PluginCallback(registrations)];

		this.registerCallbacks(callbacks);
	}

	public post<GKey extends GPluginMessageEvent['key']>(
		key: GKey,
		args: Extract<GPluginMessageEvent, { key: GKey }>['args']
	): void {
		this.figma.ui.postMessage({ key, args });
	}

	// =========================================================================
	// Helper
	// =========================================================================

	private registerCallbacks(callbacks: PluginCallback<GAppMessageEvent>[]): void {
		callbacks.forEach((callback) => {
			this.registerCallback(callback);
		});
	}

	private registerCallback(callback: PluginCallback<GAppMessageEvent>): void {
		const [typeCategory, type] = callback.type.split('.') as [string, string?];

		// Register events based on the type and type category
		const eventHandler = typeCategory === 'app' ? this.figma.ui : this.figma;
		const onKeyword = callback.once ? 'once' : 'on';
		(eventHandler[onKeyword] as any)(type as any, (...args: any[]) => {
			this.onEvent(callback, args).catch(() => {
				// do nothing
			});
		});
	}

	private async onEvent(callback: PluginCallback<GAppMessageEvent>, args: any[]): Promise<void> {
		if (callback.type === 'app.message' && args[0]?.key === callback.key) {
			await callback.callback(this, args[0].args);
		} else {
			await callback.callback(this, ...args);
		}
	}
}

export interface TFigmaPluginHandlerOptions<GAppMessageEvent extends TAppMessageEvent> {
	events?: TPluginCallbackRegistration<GAppMessageEvent>[];
}
