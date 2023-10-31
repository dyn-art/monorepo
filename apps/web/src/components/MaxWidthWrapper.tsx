import type { ClassValue } from 'clsx';
import React from 'react';
import { cn } from '@/core/utils';

const MaxWidthWrapper: React.FC<TProps> = (props) => {
	const { className = null, children } = props;

	return (
		<div className={cn('mx-auto w-full max-w-screen-xl px-2.5 md:px-20', className)}>
			{children}
		</div>
	);
};

export default MaxWidthWrapper;

interface TProps {
	className?: ClassValue;
	children: React.ReactNode;
}
