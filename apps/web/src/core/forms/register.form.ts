import {
	bitwiseFlag,
	createForm,
	FormFieldReValidateMode,
	FormFieldValidateMode
} from 'feature-form';
import * as v from 'valibot';
import { valibotAdapter } from 'validation-adapters/valibot';

import { emailValidator } from './validators';

interface TRegisterFormFields {
	email: string;
	password: string;
	name: string;
}

export const $registerForm = createForm<TRegisterFormFields>({
	fields: {
		email: {
			validationAdapter: emailValidator,
			defaultValue: ''
		},
		password: {
			validationAdapter: valibotAdapter(
				v.pipe(
					v.string(),
					v.nonEmpty('Please enter your password.'),
					v.minLength(6, 'Minimum 6 characters required!')
				)
			),
			defaultValue: ''
		},
		name: {
			validationAdapter: valibotAdapter(v.pipe(v.string(), v.nonEmpty('Please enter your name.'))),
			defaultValue: ''
		}
	},
	validateMode: bitwiseFlag(FormFieldValidateMode.OnSubmit),
	reValidateMode: bitwiseFlag(FormFieldReValidateMode.OnBlur),
	notifyOnStatusChange: false
});
