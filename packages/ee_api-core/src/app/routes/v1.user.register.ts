import { AppError } from '@ibg/openapi-router';
import bcrypt from 'bcrypt';
import { eq } from 'drizzle-orm';
import * as v from 'valibot';
import { vValidator } from 'validation-adapters/valibot';
import { db, schema } from '@/db';

import { openApiRouter } from '../router';

openApiRouter.post('/v1/user/register', {
	bodyValidator: vValidator(
		v.object({
			username: v.pipe(v.string(), v.nonEmpty()),
			email: v.pipe(v.string(), v.nonEmpty(), v.email()),
			password: v.pipe(v.string(), v.nonEmpty())
		})
	),
	handler: async (c) => {
		const { email, username, password } = c.req.valid('json');

		const existingUser = await db
			.select({ id: schema.users.id })
			.from(schema.users)
			.where(eq(schema.users.email, email))
			.limit(1);
		if (existingUser.length > 0) {
			throw new AppError('#ERR_EMAIL_TAKEN', 409, {
				description: `The specified email address (${email}) is already taken!`
			});
		}

		const hashedPassword = await bcrypt.hash(password, 10);
		await db.insert(schema.users).values({
			email,
			username,
			password: hashedPassword
		});

		c.status(200);

		return c.body(null);
	}
});
