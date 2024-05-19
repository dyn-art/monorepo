/**
 * -----------------------------------------------------------------------------
 * This file includes code derived from the project shadcn-ui/ui by \@shadcn.
 * Project Repository: https://github.com/shadcn-ui/ui/blob/main/apps/www/registry/new-york/ui/form.tsx
 *
 * Date of Import: 19 May 2024
 * -----------------------------------------------------------------------------
 * The code included in this file is licensed under the MIT License,
 * as per the original project by \@shadcn.
 * For the license text, see: https://github.com/shadcn-ui/ui/blob/main/LICENSE.md
 * -----------------------------------------------------------------------------
 */

import type * as LabelPrimitive from '@radix-ui/react-label';
import { Slot } from '@radix-ui/react-slot';
import * as React from 'react';
import {
	Controller,
	FormProvider,
	useFormContext,
	type ControllerProps,
	type FieldPath,
	type FieldValues
} from 'react-hook-form';
import { Label } from '@/components/primitive';
import { cn } from '@/utils';

const Form = FormProvider;

interface FormFieldContextValue<
	TFieldValues extends FieldValues = FieldValues,
	TName extends FieldPath<TFieldValues> = FieldPath<TFieldValues>
> {
	name: TName;
}

const FormFieldContext = React.createContext<FormFieldContextValue>({} as FormFieldContextValue);

const FormField = <
	TFieldValues extends FieldValues = FieldValues,
	TName extends FieldPath<TFieldValues> = FieldPath<TFieldValues>
>({
	...props
}: ControllerProps<TFieldValues, TName>) => {
	return (
		<FormFieldContext.Provider value={{ name: props.name }}>
			<Controller {...props} />
		</FormFieldContext.Provider>
	);
};

const useFormField = () => {
	const fieldContext = React.useContext(FormFieldContext);
	const itemContext = React.useContext(FormItemContext);
	const { getFieldState, formState } = useFormContext();

	const fieldState = getFieldState(fieldContext.name, formState);

	const { id } = itemContext;

	return {
		id,
		name: fieldContext.name,
		formItemId: `${id}-form-item`,
		formDescriptionId: `${id}-form-item-description`,
		formMessageId: `${id}-form-item-message`,
		...fieldState
	};
};

interface FormItemContextValue {
	id: string;
}

const FormItemContext = React.createContext<FormItemContextValue>({} as FormItemContextValue);

const FormItem = React.forwardRef<HTMLDivElement, React.HTMLAttributes<HTMLDivElement>>(
	({ className, ...props }, ref) => {
		const id = React.useId();

		return (
			<FormItemContext.Provider value={{ id }}>
				<div className={cn('space-y-2', className)} ref={ref} {...props} />
			</FormItemContext.Provider>
		);
	}
);
FormItem.displayName = 'FormItem';

const FormLabel = React.forwardRef<
	React.ElementRef<typeof LabelPrimitive.Root>,
	React.ComponentPropsWithoutRef<typeof LabelPrimitive.Root>
>(({ className, ...props }, ref) => {
	const { error, formItemId } = useFormField();

	return (
		<Label
			className={cn(error && 'text-destructive', className)}
			htmlFor={formItemId}
			ref={ref}
			{...props}
		/>
	);
});
FormLabel.displayName = 'FormLabel';

const FormControl = React.forwardRef<
	React.ElementRef<typeof Slot>,
	React.ComponentPropsWithoutRef<typeof Slot>
>(({ ...props }, ref) => {
	const { error, formItemId, formDescriptionId, formMessageId } = useFormField();

	return (
		<Slot
			aria-describedby={!error ? `${formDescriptionId}` : `${formDescriptionId} ${formMessageId}`}
			aria-invalid={Boolean(error)}
			id={formItemId}
			ref={ref}
			{...props}
		/>
	);
});
FormControl.displayName = 'FormControl';

const FormDescription = React.forwardRef<
	HTMLParagraphElement,
	React.HTMLAttributes<HTMLParagraphElement>
>(({ className, ...props }, ref) => {
	const { formDescriptionId } = useFormField();

	return (
		<p
			className={cn('text-muted-foreground text-[0.8rem]', className)}
			id={formDescriptionId}
			ref={ref}
			{...props}
		/>
	);
});
FormDescription.displayName = 'FormDescription';

const FormMessage = React.forwardRef<
	HTMLParagraphElement,
	React.HTMLAttributes<HTMLParagraphElement>
>(({ className, children, ...props }, ref) => {
	const { error, formMessageId } = useFormField();
	const body = error ? String(error.message) : children;

	if (!body) {
		return null;
	}

	return (
		<p
			className={cn('text-destructive text-[0.8rem] font-medium', className)}
			id={formMessageId}
			ref={ref}
			{...props}
		>
			{body}
		</p>
	);
});
FormMessage.displayName = 'FormMessage';

export {
	Form,
	FormControl,
	FormDescription,
	FormField,
	FormItem,
	FormLabel,
	FormMessage,
	useFormField
};
