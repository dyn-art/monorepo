import { FigmaPluginHandler } from '@dyn/figma-handler/plugin';

import type { TAppMessageEvents, TPluginHandler, TPluginMessageEvents } from '../types';
import {
	documentChangeEvent,
	intermediateFormatExport,
	selectionChange,
	uiRouteChange
} from './events';

export const pluginHandler: TPluginHandler = new FigmaPluginHandler<
	TAppMessageEvents,
	TPluginMessageEvents
>(figma, {
	events: [documentChangeEvent, intermediateFormatExport, selectionChange, uiRouteChange]
});
