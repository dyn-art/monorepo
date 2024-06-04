import { createState } from 'feature-state';

import type { EAppRoutes } from '../../../types';

export const ACTIVE_APP_ROUTE = createState<EAppRoutes | null>(null);
export const SELECTED_NODE_IDS = createState<string[]>([]);
