import {
	bitwiseFlag,
	createForm,
	FormFieldReValidateMode,
	FormFieldValidateMode
} from 'feature-form';
import { valibotValidator } from 'feature-form-validators/valibot';
import * as v from 'valibot';

import { emailValidator } from './validators';

interface TRegisterFormFields {
	email: string;
	password: string;
	name: string;
}

export const $registerForm = createForm<TRegisterFormFields>({
	fields: {
		email: {
			validator: emailValidator,
			defaultValue: ''
		},
		password: {
			validator: valibotValidator(
				v.pipe(
					v.string(),
					v.nonEmpty('Please enter your password.'),
					v.minLength(6, 'Minimum 6 characters required!')
				)
			),
			defaultValue: ''
		},
		name: {
			validator: valibotValidator(v.pipe(v.string(), v.nonEmpty('Please enter your name.'))),
			defaultValue: ''
		}
	},
	validateMode: bitwiseFlag(FormFieldValidateMode.OnSubmit),
	reValidateMode: bitwiseFlag(FormFieldReValidateMode.OnBlur),
	notifyOnStatusChange: false
});
