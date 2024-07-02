import {
	bitwiseFlag,
	createForm,
	FormFieldReValidateMode,
	FormFieldValidateMode
} from 'feature-form';
import { valibotValidator } from 'feature-form-validators/valibot';
import { withGlobalBind } from 'feature-react/state';
import * as v from 'valibot';

type TLoginFormFields = {
	email: string;
	password: string;
};

export const $loginForm = withGlobalBind(
	'_form',
	createForm<TLoginFormFields>({
		fields: {
			email: {
				validator: valibotValidator(
					v.pipe(
						v.string(),
						v.nonEmpty('Please enter your email.'),
						v.email('The email is badly formatted.'),
						v.maxLength(30, 'Your email is too long.')
					)
				),
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
	})
);
