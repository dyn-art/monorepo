const { drizzle } = require('drizzle-orm/postgres-js');
const { migrate } = require('drizzle-orm/postgres-js/migrator');
const postgres = require('postgres');

// Only load .env in development
const nodeEnv = process.env.NODE_ENV ?? 'local';
if (nodeEnv === 'local') {
	require('dotenv').config({ path: `.env.${nodeEnv}` });
	console.log(`Loaded dotenv from '.env.${nodeEnv}'.`);
}

(async () => {
	const dbUrl = process.env.DB_URL;
	if (dbUrl == null) {
		throw Error('Failed to resolve database url!');
	}

	const migrationClient = postgres(process.env.DB_URL, { max: 1 });

	await migrate(drizzle(migrationClient), {
		migrationsFolder: './drizzle/migrations'
	});

	migrationClient.end();
})().catch((e) => {
	console.error('Failed to run database migrations by exception: ', e);
});
