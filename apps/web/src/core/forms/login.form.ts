import {
	bitwiseFlag,
	createForm,
	FormFieldReValidateMode,
	FormFieldValidateMode
} from 'feature-form';
import { valibotValidator } from 'feature-form-validators/valibot';
import * as v from 'valibot';

import { emailValidator } from './validators';

interface TLoginFormFields {
	email: string;
	password: string;
}

export const $loginForm = createForm<TLoginFormFields>({
	fields: {
		email: {
			validator: emailValidator,
			defaultValue: ''
		},
		password: {
			validator: valibotValidator(v.pipe(v.string(), v.nonEmpty('Please enter your password.'))),
			defaultValue: ''
		}
	},
	validateMode: bitwiseFlag(FormFieldValidateMode.OnSubmit),
	reValidateMode: bitwiseFlag(FormFieldReValidateMode.OnBlur),
	notifyOnStatusChange: false
});
