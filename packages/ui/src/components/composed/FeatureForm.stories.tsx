import type { Meta, StoryObj } from '@storybook/react';
import {
	bitwiseFlag,
	createForm,
	createValidator,
	FormFieldReValidateMode,
	FormFieldValidateMode,
	valibotValidator
} from 'feature-form';
import { useForm } from 'feature-react/form';
import { maxLength, minLength, pipe, regex, string } from 'valibot';

import { Input } from '../primitive';
import {
	FormControl,
	FormDescription,
	FormField,
	FormItem,
	FormLabel,
	FormMessage
} from './FeatureForm';

type TFormData = {
	firstName: string;
	lastName: string;
};

const nameValidator = valibotValidator(
	pipe(string(), minLength(2), maxLength(10), regex(/^([^0-9]*)$/))
);

const $form = createForm<TFormData>({
	fields: {
		firstName: {
			validator: nameValidator,
			defaultValue: ''
		},
		lastName: {
			validator: nameValidator.clone().append(
				createValidator([
					{
						key: 'jeff',
						validate: (formField) => {
							if (formField.get()?.includes('Jeff')) {
								formField.status.registerNextError({
									code: 'jeff-not-last-name',
									message: 'Jeff is not a last name!'
								});
							}
						}
					}
				])
			),
			defaultValue: ''
		}
	},
	validateMode: bitwiseFlag(FormFieldValidateMode.OnChange),
	reValidateMode: bitwiseFlag(FormFieldReValidateMode.OnChange)
});

const meta = {
	title: 'ui/FeatureForm',
	tags: ['autodocs'],
	argTypes: {},
	render: (args) => {
		const { field } = useForm($form);

		return (
			<div>
				<FormField formField={field('firstName')}>
					{({ fieldData }) => (
						<FormItem>
							<FormLabel />
							<FormControl>
								<Input placeholder="Jeff" {...fieldData} />
							</FormControl>
							<FormDescription>A creative First Name description.</FormDescription>
							<FormMessage />
						</FormItem>
					)}
				</FormField>
				<FormField formField={field('lastName')}>
					{({ fieldData }) => (
						<FormItem>
							<FormLabel />
							<FormControl>
								<Input placeholder="Placeholder" {...fieldData} />
							</FormControl>
							<FormDescription>A creative Last Name description.</FormDescription>
							<FormMessage />
						</FormItem>
					)}
				</FormField>
			</div>
		);
	},
	parameters: {
		layout: 'centered'
	}
} satisfies Meta;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {};
