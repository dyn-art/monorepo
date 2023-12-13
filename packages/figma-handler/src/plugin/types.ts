export type TPluginEventRegistration<GAppMessageEvent extends TAppMessageEvent = TAppMessageEvent> =
	GAppMessageEvent extends TAppMessageEvent
		? {
				[K in keyof TPluginEventTypes<TAppMessageEvent>]: TPluginEventRegistrationBase<
					GAppMessageEvent,
					K
				>;
		  }[keyof TPluginEventTypes<GAppMessageEvent>]
		: never;

export interface TPluginEventRegistrationBase<
	GAppMessageEvent extends TAppMessageEvent,
	GEventType extends keyof TPluginEventTypes<GAppMessageEvent>,
	GPluginEventCallbackArgs extends
		TPluginEventTypes<GAppMessageEvent>[GEventType][0] = TPluginEventTypes<GAppMessageEvent>[GEventType][0],
	GPluginEventCallbackReturnValue extends
		TPluginEventTypes<GAppMessageEvent>[GEventType][1] = TPluginEventTypes<GAppMessageEvent>[GEventType][1],
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
	GEventType extends keyof TPluginEventTypes<GAppMessageEvent>,
	GPluginEventCallbackArgs extends
		TPluginEventTypes<GAppMessageEvent>[GEventType][0] = TPluginEventTypes<GAppMessageEvent>[GEventType][0]
> = GPluginEventCallbackArgs[0] extends {
	key: infer TKey;
}
	? TKey
	: string | undefined;

export interface TPluginEventTypes<GAppMessageEvent extends TAppMessageEvent> {
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

// const test: TPluginEventRegistration<TAppMessageEvents> = {
// 	type: 'run',
// 	key: undefined,
// 	callback: async (instance, event) => {
// 		// Event received by Figma
// 	}
// };

// const test2: TPluginEventRegistration<TAppMessageEvents> = {
// 	type: 'app.message',
// 	key: 'on-event-1',
// 	callback: async (instance, event) => {
// 		// Event received by plugin part
// 	}
// };

// const test3: TPluginEventRegistration<TAppMessageEvents> = {
// 	type: 'textreview',
// 	key: undefined,
// 	callback: async (instance, event) => {
// 		// Event received by Figma
// 		return [];
// 	}
// };
