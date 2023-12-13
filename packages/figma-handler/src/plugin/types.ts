export type TPluginEventMeta<GAppMessageEvent extends TAppMessageEvent = TAppMessageEvent> =
	GAppMessageEvent extends TAppMessageEvent
		? {
				[K in keyof TPluginEvents<TAppMessageEvent>]: TPluginEventMetaBase<GAppMessageEvent, K>;
		  }[keyof TPluginEvents<GAppMessageEvent>]
		: never;

export interface TPluginEventMetaBase<
	GAppMessageEvent extends TAppMessageEvent,
	GEventType extends keyof TPluginEvents<GAppMessageEvent>,
	GPluginEventCallbackArgs extends
		TPluginEvents<GAppMessageEvent>[GEventType][0] = TPluginEvents<GAppMessageEvent>[GEventType][0],
	GPluginEventCallbackReturnValue extends
		TPluginEvents<GAppMessageEvent>[GEventType][1] = TPluginEvents<GAppMessageEvent>[GEventType][1],
	GPluginEventKey extends TPluginEventKey<GAppMessageEvent, GEventType> = TPluginEventKey<
		GAppMessageEvent,
		GEventType
	>
> {
	key: GPluginEventKey;
	type: GEventType;
	once?: boolean;
	callback: (
		instance: any,
		...args: GPluginEventCallbackArgs[0] extends {
			key: GPluginEventKey;
			args: infer TArgs;
		}
			? [args: TArgs]
			: GPluginEventCallbackArgs
	) => GPluginEventCallbackReturnValue;
}

export type TPluginEventKey<
	GAppMessageEvent extends TAppMessageEvent,
	GEventType extends keyof TPluginEvents<GAppMessageEvent>,
	GPluginEventCallbackArgs extends
		TPluginEvents<GAppMessageEvent>[GEventType][0] = TPluginEvents<GAppMessageEvent>[GEventType][0]
> = GPluginEventCallbackArgs[0] extends {
	key: infer TKey;
}
	? TKey
	: string | undefined;

export interface TPluginEvents<GAppMessageEvent extends TAppMessageEvent> {
	// Events received from Figma
	'run': [[event: RunEvent], Promise<void>];
	'drop': [[event: DropEvent], Promise<void>];
	'documentchange': [[event: DocumentChangeEvent], Promise<void>];
	'textreview': [[event: TextReviewEvent], Promise<TextReviewRange[]>];
	'selectionchange': [[], Promise<void>];
	'currentpagechange': [[], Promise<void>];
	'close': [[], Promise<void>];
	'timerstart': [[], Promise<void>];
	'timerstop': [[], Promise<void>];
	'timerpause': [[], Promise<void>];
	'timerresume': [[], Promise<void>];
	'timeradjust': [[], Promise<void>];
	'timerdone': [[], Promise<void>];

	// Events received from app part
	'app.message': [[event: GAppMessageEvent], Promise<void>];
}

export interface TAppMessageEvent {
	key: string;
	args: any;
}

// Testing

// export interface TOnAppMessageEvent2 extends TAppMessageEvent {
// 	key: 'on-event-2';
// 	args: {
// 		arg1: number;
// 		arg2: string;
// 	};
// }

// export interface TOnAppMessageEvent1 extends TAppMessageEvent {
// 	key: 'on-event-1';
// 	args: {
// 		arg1: null;
// 		arg2: number;
// 		arg3: string;
// 	};
// }

// export type TAppMessageEvents = TOnAppMessageEvent1 | TOnAppMessageEvent2;

// const test: TPluginEventMeta<TAppMessageEvents> = {
// 	type: 'run',
// 	key: undefined,
// 	callback: async (instance, event) => {
// 		// TODO
// 	}
// };

// const test2: TPluginEventMeta<TAppMessageEvents> = {
// 	type: 'app.message',
// 	key: 'on-event-1',
// 	callback: async (instance, event) => {
// 		// TODO
// 	}
// };

// const test3: TPluginEventMeta<TAppMessageEvents> = {
// 	type: 'textreview',
// 	key: undefined,
// 	callback: async (instance, event) => {
// 		// TODO
// 		return [];
// 	}
// };
