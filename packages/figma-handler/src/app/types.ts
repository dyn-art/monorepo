export type TAppEventRegistration<
	GPluginMessageEvent extends TPluginMessageEvent = TPluginMessageEvent
> = GPluginMessageEvent extends TPluginMessageEvent
	? {
			[K in keyof TAppEventTypes<TPluginMessageEvent>]: TAppEventRegistrationBase<
				GPluginMessageEvent,
				K
			>;
	  }[keyof TAppEventTypes<GPluginMessageEvent>]
	: never;

export interface TAppEventRegistrationBase<
	GPluginMessageEvent extends TPluginMessageEvent,
	GEventType extends keyof TAppEventTypes<GPluginMessageEvent>,
	GAppEventCallbackArgs extends
		TAppEventTypes<GPluginMessageEvent>[GEventType][0] = TAppEventTypes<GPluginMessageEvent>[GEventType][0],
	GAppEventCallbackReturnValue extends
		TAppEventTypes<GPluginMessageEvent>[GEventType][1] = TAppEventTypes<GPluginMessageEvent>[GEventType][1],
	GAppEventKey extends TAppEventKey<GPluginMessageEvent, GEventType> = TAppEventKey<
		GPluginMessageEvent,
		GEventType
	>
> {
	key: GAppEventKey;
	type: GEventType;
	once?: boolean;
	callback: (
		instance: any,
		...args: GAppEventCallbackArgs[0] extends {
			key: GAppEventKey;
			args: infer TArgs;
		}
			? [
					args: {
						pluginId: string;
					} & TArgs
			  ]
			: GAppEventCallbackArgs
	) => GAppEventCallbackReturnValue;
}

export type TAppEventKey<
	GPluginMessageEvent extends TPluginMessageEvent,
	GEventType extends keyof TAppEventTypes<GPluginMessageEvent>,
	GAppEventCallbackArgs extends
		TAppEventTypes<GPluginMessageEvent>[GEventType][0] = TAppEventTypes<GPluginMessageEvent>[GEventType][0]
> = GAppEventCallbackArgs[0] extends {
	key: infer TKey;
}
	? TKey
	: string | undefined;

export type TAppEventTypes<
	GPluginMessageEvent extends TPluginMessageEvent,
	GWindowEventKeys extends keyof WindowEventMap = keyof WindowEventMap
> = {
	// Events received from Window
	[K in GWindowEventKeys]: [[event: WindowEventMap[K]], Promise<void>];
} & {
	// Events received from plugin part
	'plugin.message': [[event: GPluginMessageEvent], Promise<void>];
};

export interface TPluginMessageEvent {
	key: string;
	args: unknown;
}

// Testing

// export interface TOnPluginMessageEvent2 extends TPluginMessageEvent {
// 	key: 'on-event-2';
// 	args: {
// 		arg1: number;
// 		arg2: string;
// 	};
// }

// export interface TOnPluginMessageEvent1 extends TPluginMessageEvent {
// 	key: 'on-event-1';
// 	args: {
// 		arg1: null;
// 		arg2: number;
// 		arg3: string;
// 	};
// }

// export type TPluginMessageEvents = TOnPluginMessageEvent1 | TOnPluginMessageEvent2;

// const test: TAppEventRegistration<TPluginMessageEvents> = {
// 	type: 'DOMContentLoaded',
// 	key: undefined,
// 	callback: async (instance, event) => {
// 		// Event received by Window
// 	}
// };

// const test2: TAppEventRegistration<TPluginMessageEvents> = {
// 	type: 'plugin.message',
// 	key: 'on-event-1',
// 	callback: async (instance, event) => {
// 		// Event received by plugin part
// 	}
// };
