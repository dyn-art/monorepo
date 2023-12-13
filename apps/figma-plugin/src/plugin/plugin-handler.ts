import { FigmaPluginHandler } from '@dyn/figma-handler/plugin';

import type { TPluginHandler } from '../types';
import { events } from './events';

export const pluginHandler: TPluginHandler = new FigmaPluginHandler(figma, { events });
