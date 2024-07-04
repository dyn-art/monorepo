// Only load .env in development
const nodeEnv = process.env.NODE_ENV ?? 'local';
if (nodeEnv === 'local') {
	require('dotenv').config({ path: `.env.${nodeEnv}` });
	console.log(`Loaded dotenv from '.env.${nodeEnv}'.`);
}

/** @type { import("drizzle-kit").Config } */
export default {
	schema: './src/db/schema.ts',
	out: './drizzle/migrations',
	dialect: 'postgresql',
	dbCredentials: {
		url: process.env.DB_URL
	},
	verbose: true,
	strict: true
};
