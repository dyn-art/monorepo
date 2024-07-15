import * as v from 'valibot';
import { vValidator } from 'validation-adapters/valibot';

export const emailValidator = vValidator(
	v.pipe(
		v.string(),
		v.nonEmpty('Please enter your email.'),
		v.email('The email is badly formatted.'),
		v.maxLength(30, 'Your email is too long.')
	)
);
