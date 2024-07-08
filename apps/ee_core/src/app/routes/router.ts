import { createExpressOpenApiRouter } from '@ibg/openapi-router';
import { Router } from 'express';
import type { paths } from '@dyn/types/core';

export const router: Router = Router();
export const openApiRouter = createExpressOpenApiRouter<paths>(router);
