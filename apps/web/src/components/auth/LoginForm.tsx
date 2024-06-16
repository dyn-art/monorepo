'use client';

import Link from 'next/link';
import React from 'react';
import { Button, GithubIcon, GoogleIcon } from '@dyn/ui';

import { AuthFormWrapper } from './AuthFormWrapper';

export const LoginForm: React.FC = () => {
	return (
		<AuthFormWrapper
			headerLabel="Sign in to your account"
			backChildren={
				<p className="font-body mt-4 text-center text-sm text-gray-500">
					Not a member?{' '}
					<Link
						href="/auth/register"
						className="ml-1 font-semibold leading-6 text-indigo-600 hover:text-indigo-500"
					>
						Start for free
					</Link>
				</p>
			}
		>
			<p>Hello World</p>

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
