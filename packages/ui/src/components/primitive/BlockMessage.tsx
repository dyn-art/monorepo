import { cva } from 'class-variance-authority';
import React from 'react';
import { cn } from '@/utils';

import { AlertCircleIcon, CheckCircleIcon, ExclamationTriangleIcon } from './icon';

const blockMessageVariants = cva('flex items-center gap-x-2 rounded-md p-3 text-sm font-body', {
	variants: {
		variant: {
			success: 'bg-emerald-500/15 text-emerald-700',
			warn: 'bg-yellow-500/15 text-yellow-700',
			error: 'bg-destructive/15 text-destructive'
		}
	}
});

const icons = {
	success: <CheckCircleIcon className="h-4 w-4" />,
	warn: <ExclamationTriangleIcon className="h-4 w-4" />,
	error: <AlertCircleIcon className="h-4 w-4" />
};

export const BlockMessage: React.FC<TBlockMessageProps> = (props) => {
	const { variant, children, className } = props;
	return (
		<div className={cn(blockMessageVariants({ variant }), className)}>
			{icons[variant]}
			{typeof children === 'string' ? <p>{children}</p> : children}
		</div>
	);
};

interface TBlockMessageProps extends React.HTMLAttributes<HTMLDivElement> {
	variant: 'success' | 'warn' | 'error';
	children: React.ReactNode | string;
	className?: string;
}
