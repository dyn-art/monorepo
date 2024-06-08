import { FigmaPluginHandler } from 'figma-connect/plugin';

import type { TFromAppMessageEvents, TFromPluginMessageEvents } from '../types';
import {
	documentChangeEvent,
	intermediateFormatExport,
	selectionChange,
	uiRouteChange
} from './events';

export const pluginHandler = new FigmaPluginHandler<
	TFromAppMessageEvents,
	TFromPluginMessageEvents
>(figma, {
	events: [documentChangeEvent, intermediateFormatExport, selectionChange, uiRouteChange]
});
