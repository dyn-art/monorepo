export type TAppEventMeta<GPluginMessageEvent extends TPluginMessageEvent = TPluginMessageEvent> =
	GPluginMessageEvent extends TPluginMessageEvent
		? {
				[K in keyof TAppEvents<TPluginMessageEvent>]: TAppEventMetaBase<GPluginMessageEvent, K>;
		  }[keyof TAppEvents<GPluginMessageEvent>]
		: never;

export interface TAppEventMetaBase<
	GPluginMessageEvent extends TPluginMessageEvent,
	GEventType extends keyof TAppEvents<GPluginMessageEvent>,
	GAppEventCallbackArgs extends
		TAppEvents<GPluginMessageEvent>[GEventType][0] = TAppEvents<GPluginMessageEvent>[GEventType][0],
	GAppEventCallbackReturnValue extends
		TAppEvents<GPluginMessageEvent>[GEventType][1] = TAppEvents<GPluginMessageEvent>[GEventType][1],
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
			? [args: TArgs]
			: GAppEventCallbackArgs
	) => GAppEventCallbackReturnValue;
}

export type TAppEventKey<
	GPluginMessageEvent extends TPluginMessageEvent,
	GEventType extends keyof TAppEvents<GPluginMessageEvent>,
	GAppEventCallbackArgs extends
		TAppEvents<GPluginMessageEvent>[GEventType][0] = TAppEvents<GPluginMessageEvent>[GEventType][0]
> = GAppEventCallbackArgs[0] extends {
	key: infer TKey;
}
	? TKey
	: string | undefined;

export type TAppEvents<
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
	args: any;
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

// const test: TAppEventMeta<TPluginMessageEvents> = {
// 	type: 'DOMContentLoaded',
// 	key: undefined,
// 	callback: async (instance, event) => {
// 		// TODO
// 	}
// };

// const test2: TAppEventMeta<TPluginMessageEvents> = {
// 	type: 'plugin.message',
// 	key: 'on-event-1',
// 	callback: async (instance, event) => {
// 		// TODO
// 	}
// };
