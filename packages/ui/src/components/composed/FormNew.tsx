'use client';

import type * as LabelPrimitive from '@radix-ui/react-label';
import { TFormField } from 'feature-form';
import { useGlobalState } from 'feature-react/state';
import React from 'react';
import { cn } from '@/utils';

import { Label, Slot } from '../primitive';

export const FormFieldContext = React.createContext<TFormFieldWithId | null>(null);

interface TFormFieldWithId<GValue = any> extends TFormField<GValue> {
	id: string;
}

export function useFormFieldContext(): TFormFieldWithId {
	const formField = React.useContext(FormFieldContext);
	if (formField == null) {
		throw Error('useFormFieldContext() has to be used within <FormFieldContext.Provider>');
	}
	return formField;
}

export const FormField = React.forwardRef<HTMLDivElement, TFormFieldProps<any>>((props, ref) => {
	const { formField, className, ...other } = props;
	const id = React.useId();

	return (
		<FormFieldContext.Provider value={Object.assign(formField, { id })}>
			<div className={cn('space-y-2', className)} ref={ref} {...other} />
		</FormFieldContext.Provider>
	);
});
FormField.displayName = 'FormField';

export interface TFormFieldProps<GValue> extends React.HTMLAttributes<HTMLDivElement> {
	formField: TFormField<GValue>;
}

export const FormLabel = React.forwardRef<
	React.ElementRef<typeof LabelPrimitive.Root>,
	React.ComponentPropsWithoutRef<typeof LabelPrimitive.Root>
>((props, ref) => {
	const { className, ...other } = props;
	const formField = useFormFieldContext();
	const status = useGlobalState(formField.status);

	return (
		<Label
			className={cn(status.type === 'INVALID' && 'text-destructive', className)}
			htmlFor={formField.id}
			ref={ref}
			{...other}
		/>
	);
});
FormLabel.displayName = 'FormLabel';

export const FormControl = React.forwardRef<
	React.ElementRef<typeof Slot>,
	React.ComponentPropsWithoutRef<typeof Slot>
>((props, ref) => {
	const formField = useFormFieldContext();
	const status = useGlobalState(formField.status);

	return (
		<Slot
			aria-describedby={
				status.type !== 'INVALID'
					? `${formField.id}-form-item-description`
					: `${formField.id}-form-item-description ${formField.id}-form-item-message`
			}
			aria-invalid={status.type === 'INVALID'}
			id={formField.key}
			ref={ref}
			{...props}
		/>
	);
});
FormControl.displayName = 'FormControl';

export const FormDescription = React.forwardRef<
	HTMLParagraphElement,
	React.HTMLAttributes<HTMLParagraphElement>
>((props, ref) => {
	const { className, ...other } = props;
	const formField = useFormFieldContext();

	return (
		<p
			className={cn('text-muted-foreground text-[0.8rem]', className)}
			id={`${formField.id}-form-item-description`}
			ref={ref}
			{...other}
		/>
	);
});
FormDescription.displayName = 'FormDescription';

export const FormMessage = React.forwardRef<
	HTMLParagraphElement,
	React.HTMLAttributes<HTMLParagraphElement>
>((props, ref) => {
	const { children, className, ...other } = props;
	const formField = useFormFieldContext();
	const status = useGlobalState(formField.status);
	const body = status.type === 'INVALID' ? String(status.errors[0]?.message) : children;

	if (body == null) {
		return null;
	}

	return (
		<p
			className={cn('text-destructive text-[0.8rem] font-medium', className)}
			id={formField.id}
			ref={ref}
			{...other}
		>
			{body}
		</p>
	);
});
FormMessage.displayName = 'FormMessage';
