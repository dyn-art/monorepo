import { drizzle } from 'drizzle-orm/postgres-js';
import postgres from 'postgres';
import { dbConfig } from '@/environment';

import * as schema from './schema';

const client = postgres(dbConfig.url);
export const db = drizzle(client, { schema, logger: true });

export * as schema from './schema';
