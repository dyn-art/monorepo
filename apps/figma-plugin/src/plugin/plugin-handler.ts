import { FigmaPluginHandler } from 'figma-connect/plugin';

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
