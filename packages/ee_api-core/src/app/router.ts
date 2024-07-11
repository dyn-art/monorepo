import { createHonoOpenApiRouter } from '@ibg/openapi-router';
import { Hono } from 'hono';
import { type paths } from '@dyn/types/core';

export const router = new Hono();
export const openApiRouter = createHonoOpenApiRouter<paths>(router);
