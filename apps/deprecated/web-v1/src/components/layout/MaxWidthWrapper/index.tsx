import type { ClassValue } from 'clsx';
import React from 'react';
import { cn } from '@dyn/ui';

const MAX_W_SCREEN = 1280; // px
const PADDING = 160; // px
export const CONTENT_WIDTH = MAX_W_SCREEN - PADDING;

const MaxWidthWrapper: React.FC<TProps> = (props) => {
	const { className = null, children } = props;

	return (
		<div className={cn('mx-auto w-full max-w-screen-xl px-2.5 md:px-20', className)}>
			{children}
		</div>
	);
};

export { MaxWidthWrapper };

interface TProps {
	className?: ClassValue;
	children: React.ReactNode;
}
