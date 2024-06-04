import { FigmaAppHandler } from 'figma-connect/app';

import type { TAppHandler } from '../types';

export const appHandler: TAppHandler = new FigmaAppHandler(parent);
