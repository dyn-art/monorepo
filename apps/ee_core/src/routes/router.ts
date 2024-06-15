import { Router } from 'express';
import { createOpenApiRouter } from 'openapi-express';
import type { paths } from '@dyn/types/core';

export const router: Router = Router();
export const openApiRouter = createOpenApiRouter<paths>(router);
