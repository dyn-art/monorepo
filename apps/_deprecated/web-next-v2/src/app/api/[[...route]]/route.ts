import { Hono } from 'hono';
import { handle } from 'hono/vercel';
import { createApp } from '@dyn/api-core';

// export const runtime = 'edge';

const app = createApp(new Hono().basePath('/api'));

export const GET = handle(app);
export const POST = handle(app);
