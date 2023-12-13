export class FigmaAppHandler {
	private readonly _parent: Window;

	constructor(parentInstance: Window) {
		this._parent = parentInstance;

		// figma.ui.onmessage = (message) => {
		// 	// TODO
		// };

		// figma.on('');

		// figma.ui.postMessage('');
	}

	public get parent() {
		return this._parent;
	}
}

// =============================================================================
// Plugin
// =============================================================================

// type: app.message | figma-events
// key: xyz

type TPluginEventMeta<GAppMessageEvent extends TAppMessageEvent = TAppMessageEvent> =
	GAppMessageEvent extends TAppMessageEvent
		? {
				[K in keyof TPluginEvents<TAppMessageEvent>]: TPluginEventMetaBase<GAppMessageEvent>;
		  }[keyof TPluginEvents<GAppMessageEvent>]
		: never;

interface TPluginEventMetaBase<
	GAppMessageEvent extends TAppMessageEvent,
	GEventType extends keyof TPluginEvents<GAppMessageEvent> = keyof TPluginEvents<GAppMessageEvent>,
	GPluginEventArgs extends
		TPluginEvents<GAppMessageEvent>[GEventType][0] = TPluginEvents<GAppMessageEvent>[GEventType][0],
	GPluginEventResponse extends
		TPluginEvents<GAppMessageEvent>[GEventType][1] = TPluginEvents<GAppMessageEvent>[GEventType][1],
	GPluginEventKey extends TPluginEventKey<
		GAppMessageEvent,
		GEventType,
		GPluginEventArgs
	> = TPluginEventKey<GAppMessageEvent, GEventType, GPluginEventArgs>
> {
	key: GPluginEventKey;
	type: GEventType;
	once?: boolean;
	callback: (
		instance: any,
		...args: GPluginEventArgs[0] extends {
			key: GPluginEventKey;
			args: infer TArgs;
		}
			? [args: TArgs]
			: GPluginEventArgs
	) => GPluginEventResponse;
}

type TPluginEventKey<
	GAppMessageEvent extends TAppMessageEvent,
	GEventType extends keyof TPluginEvents<GAppMessageEvent>,
	GPluginEventArgs extends TPluginEvents<GAppMessageEvent>[GEventType][0]
> = GPluginEventArgs[0] extends {
	key: infer TKey;
}
	? TKey
	: string | undefined;

export interface TPluginEvents<GAppMessageEvent extends TAppMessageEvent> {
	// Figma events
	'run': [[event: RunEvent], Promise<void> | void];
	'drop': [[event: DropEvent], Promise<void> | void];
	'documentchange': [[event: DocumentChangeEvent], Promise<void> | void];
	'textreview': [[event: TextReviewEvent], Promise<TextReviewRange[]> | TextReviewRange[]];
	'selectionchange': [[], Promise<void> | void];
	'currentpagechange': [[], Promise<void> | void];
	'close': [[], Promise<void> | void];
	'timerstart': [[], Promise<void> | void];
	'timerstop': [[], Promise<void> | void];
	'timerpause': [[], Promise<void> | void];
	'timerresume': [[], Promise<void> | void];
	'timeradjust': [[], Promise<void> | void];
	'timerdone': [[], Promise<void> | void];

	// Custom events
	'app.message': [[event: GAppMessageEvent], Promise<void> | void];
}

interface TAppMessageEvent {
	key: string;
	args: any;
}

// Testing
export interface TOnAppMessageEvent2 extends TAppMessageEvent {
	key: 'on-event-2';
	args: {
		arg1: number;
		arg2: string;
	};
}

export interface TOnAppMessageEvent1 extends TAppMessageEvent {
	key: 'on-event-1';
	args: {
		arg1: null;
		arg2: number;
		arg3: string;
	};
}

export type TAppMessageEvents = TOnAppMessageEvent1 | TOnAppMessageEvent2;

const test: TPluginEventMeta<TAppMessageEvents> = {
	type: 'run',
	callback: (instance, event) => {
		// TODO
	}
};

const test2: TPluginEventMeta<TAppMessageEvents> = {
	type: 'app.message',
	key: 'on-event-1',
	callback: (instance, event) => {
		// TODO
	}
};

// =============================================================================
// App
// =============================================================================

// type: plugin.message | browser-events
// key: xyz
