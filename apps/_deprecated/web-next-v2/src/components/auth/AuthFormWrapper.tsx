import React from 'react';
import { Card, CardContent, cn, LogoIcon } from '@dyn/ui';

export const AuthFormWrapper: React.FC<TProps> = (props) => {
	const { children, headerLabel, backChildren, className } = props;

	return (
		<div className="flex w-full flex-col items-center gap-y-4">
			<LogoIcon className="h-10 w-10" />
			<h2 className="font-display mt-2 text-center text-2xl font-bold text-gray-900">
				{headerLabel}
			</h2>
			<Card className="mt-8 sm:mx-auto sm:w-full sm:max-w-[480px]">
				<CardContent className={cn('pt-6', className)}>{children}</CardContent>
			</Card>
			{backChildren}
		</div>
	);
};

interface TProps {
	children: React.ReactNode;
	headerLabel: string;
	backChildren: React.ReactNode;
	className?: string;
}
