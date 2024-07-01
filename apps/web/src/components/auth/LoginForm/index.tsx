'use client';

import { useForm } from 'feature-react/form';
import Link from 'next/link';
import React from 'react';
import {
	BlockMessage,
	Button,
	FormControl,
	FormField,
	FormItem,
	FormLabel,
	FormMessage,
	GithubIcon,
	GoogleIcon,
	Input
} from '@dyn/ui';

import { AuthFormWrapper } from '../AuthFormWrapper';
import { $loginForm } from './controller';

export const LoginForm: React.FC = () => {
	const { field, handleSubmit } = useForm($loginForm);

	return (
		<AuthFormWrapper
			headerLabel="Sign in to your account"
			backChildren={
				<p className="mt-4 text-center text-sm">
					Don't have an account?{' '}
					<Link href="#" className="underline">
						Sign up
					</Link>
				</p>
			}
		>
			<form
				onSubmit={handleSubmit({
					onInvalidSubmit: (errors) => {
						console.log({ errors });
					},
					onValidSubmit: (data) => {
						console.log({ data });
					},
					preventDefault: true
				})}
				className="space-y-6"
			>
				<div className="space-y-4">
					<FormField formField={field('email')}>
						{({ fieldData }) => (
							<FormItem>
								<FormLabel>Email</FormLabel>
								<FormControl>
									<Input {...fieldData} placeholder="john.doe@example.com" type="email" />
								</FormControl>
								<FormMessage />
							</FormItem>
						)}
					</FormField>
					<FormField formField={field('password')}>
						{({ fieldData }) => (
							<FormItem>
								<FormLabel>Password</FormLabel>
								<FormControl>
									<Input {...fieldData} placeholder="*******" type="password" />
								</FormControl>
								<FormMessage />
							</FormItem>
						)}
					</FormField>
				</div>
				<BlockMessage variant="error">This is a message in a Block Message</BlockMessage>
				<Button type="submit" className="w-full">
					Sign in
				</Button>
			</form>

			<div className="flex w-full items-center gap-x-2">
				<Button size="lg" className="w-full" variant="outline">
					<GoogleIcon className="h-6 w-6" />
				</Button>
				<Button size="lg" className="w-full" variant="outline">
					<GithubIcon className="h-6 w-6" />
				</Button>
			</div>
		</AuthFormWrapper>
	);
};
