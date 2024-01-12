import type { FigmaAppHandler } from '@dyn/figma-handler/app';
import type {
	FigmaPluginHandler,
	TAppCallbackRegistration,
	TPluginCallbackRegistration
} from '@dyn/figma-handler/plugin';

import type { TAppMessageEvents } from './app-events';
import type { TPluginMessageEvents } from './plugin-events';

export type TCustomAppCallbackRegistration = TAppCallbackRegistration<TPluginMessageEvents>;
export type TAppHandler = FigmaAppHandler<TPluginMessageEvents, TAppMessageEvents>;

export type TCustomPluginCallbackRegistration = TPluginCallbackRegistration<TAppMessageEvents>;
export type TPluginHandler = FigmaPluginHandler<TAppMessageEvents, TPluginMessageEvents>;
