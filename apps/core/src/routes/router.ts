import { Router } from 'express';
import { createOpenApiRouter } from '@dyn/openapi-router';
import type { paths } from '@dyn/types/core';

export const router: Router = Router();
export const openApiRouter = createOpenApiRouter<paths>(router);
