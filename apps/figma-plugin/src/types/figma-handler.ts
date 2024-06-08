import type { FigmaAppHandler } from 'figma-connect/app';
import type {
	FigmaPluginHandler,
	TAppCallbackRegistration,
	TPluginCallbackRegistration
} from 'figma-connect/plugin';

import type { TFromAppMessageEvents } from './app-events';
import type { TFromPluginMessageEvents } from './plugin-events';

export type TCustomAppCallbackRegistration = TAppCallbackRegistration<TFromPluginMessageEvents>;
export type TAppHandler = FigmaAppHandler<TFromPluginMessageEvents, TFromAppMessageEvents>;

export type TCustomPluginCallbackRegistration = TPluginCallbackRegistration<TFromAppMessageEvents>;
export type TPluginHandler = FigmaPluginHandler<TFromAppMessageEvents, TFromPluginMessageEvents>;
