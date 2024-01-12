import { FigmaPluginHandler } from '@dyn/figma-handler/plugin';

import type { TCustomPluginCallbackRegistration, TPluginHandler } from '../types';

export const pluginHandler: TPluginHandler = new FigmaPluginHandler(figma);

export function registerPluginEventCallback(callback: TCustomPluginCallbackRegistration): void {
	pluginHandler.register(callback);
}
