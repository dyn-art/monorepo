import {
	bitwiseFlag,
	createForm,
	FormFieldReValidateMode,
	FormFieldValidateMode
} from 'feature-form';
import * as v from 'valibot';
import { valibotAdapter } from 'validation-adapters/valibot';

import { emailValidator } from './validators';

interface TLoginFormFields {
	email: string;
	password: string;
}

export const $loginForm = createForm<TLoginFormFields>({
	fields: {
		email: {
			validationAdapter: emailValidator,
			defaultValue: ''
		},
		password: {
			validationAdapter: valibotAdapter(
				v.pipe(v.string(), v.nonEmpty('Please enter your password.'))
			),
			defaultValue: ''
		}
	},
	validateMode: bitwiseFlag(FormFieldValidateMode.OnSubmit),
	reValidateMode: bitwiseFlag(FormFieldReValidateMode.OnBlur),
	notifyOnStatusChange: false
});
