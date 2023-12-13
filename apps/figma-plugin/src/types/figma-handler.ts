import type {
	FigmaAppHandler,
	TAppCallbackRegistration as TFigmaHandlerAppCallbackRegistration,
	TPluginCallbackRegistration as TFigmaHandlerPluginCallbackRegistration
} from '@dyn/figma-handler/app';
import type { FigmaPluginHandler } from '@dyn/figma-handler/plugin';

import type { TAppMessageEvents } from './app-events';
import type { TPluginMessageEvents } from './plugin-events';

export type TAppCallbackRegistration = TFigmaHandlerAppCallbackRegistration<TPluginMessageEvents>;
export type TAppHandler = FigmaAppHandler<TPluginMessageEvents, TAppMessageEvents>;

export type TPluginCallbackRegistration =
	TFigmaHandlerPluginCallbackRegistration<TAppMessageEvents>;
export type TPluginHandler = FigmaPluginHandler<TAppMessageEvents, TPluginMessageEvents>;
