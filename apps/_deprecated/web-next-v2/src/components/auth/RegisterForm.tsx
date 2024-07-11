'use client';

import { useForm } from 'feature-react/form';
import Link from 'next/link';
import React from 'react';
import {
	AdvancedInput,
	BlockMessage,
	Button,
	FormControl,
	FormField,
	FormItem,
	FormLabel,
	FormMessage,
	GithubIcon,
	GoogleIcon
} from '@dyn/ui';
import { $registerForm } from '@/core';

import { AuthFormWrapper } from './AuthFormWrapper';

export const RegisterForm: React.FC = () => {
	const { field, handleSubmit } = useForm($registerForm);
	const isDisabled = false; // TODO: Disable if submitting
	const [blockMessage, setBlockMessage] = React.useState<{
		variant: 'error' | 'success' | 'warn';
		message: string;
	} | null>(null);

	const onSubmit = handleSubmit({
		onInvalidSubmit: (errors) => {
			console.log({ errors });
		},
		onValidSubmit: (data) => {
			console.log({ data });
		},
		preventDefault: true
	});

	return (
		<AuthFormWrapper
			headerLabel="Create an account"
			backChildren={
				<p className="mt-4 text-center text-sm">
					Already have an account?{' '}
					<Link href="/auth/login" className="underline">
						Sign in
					</Link>
				</p>
			}
		>
			<form
				// eslint-disable-next-line @typescript-eslint/no-misused-promises -- ok
				onSubmit={onSubmit}
				className="space-y-6"
			>
				<div className="space-y-4">
					<FormField formField={field('name')}>
						{(fieldData) => (
							<FormItem>
								<FormLabel>Name</FormLabel>
								<FormControl>
									{(status) => (
										<AdvancedInput
											{...fieldData}
											disabled={isDisabled}
											placeholder="John Doe"
											variant={status.type === 'INVALID' ? 'destructive' : 'default'}
										/>
									)}
								</FormControl>
								<FormMessage />
							</FormItem>
						)}
					</FormField>
					<FormField formField={field('email')}>
						{(fieldData) => (
							<FormItem>
								<FormLabel>Email</FormLabel>
								<FormControl>
									{(status) => (
										<AdvancedInput
											{...fieldData}
											disabled={isDisabled}
											placeholder="john.doe@example.com"
											type="email"
											variant={status.type === 'INVALID' ? 'destructive' : 'default'}
										/>
									)}
								</FormControl>
								<FormMessage />
							</FormItem>
						)}
					</FormField>
					<FormField formField={field('password')}>
						{(fieldData) => (
							<FormItem>
								<FormLabel>Password</FormLabel>
								<FormControl>
									{(status) => (
										<AdvancedInput
											{...fieldData}
											disabled={isDisabled}
											placeholder="*******"
											type="password"
											variant={status.type === 'INVALID' ? 'destructive' : 'default'}
										/>
									)}
								</FormControl>
								<FormMessage />
							</FormItem>
						)}
					</FormField>
				</div>
				{blockMessage != null && (
					<BlockMessage variant={blockMessage.variant}>{blockMessage.message}</BlockMessage>
				)}
				<Button type="submit" className="w-full" disabled={isDisabled}>
					Sign up
				</Button>
			</form>

			<div className="mt-6 flex w-full items-center gap-x-2">
				<Button size="lg" className="w-full" variant="outline" disabled={isDisabled}>
					<GoogleIcon className="h-6 w-6" />
				</Button>
				<Button size="lg" className="w-full" variant="outline" disabled={isDisabled}>
					<GithubIcon className="h-6 w-6" />
				</Button>
			</div>
		</AuthFormWrapper>
	);
};
