import { cva } from 'class-variance-authority';
import React from 'react';
import { cn } from '@/utils';

const containerVariants = cva('mx-auto w-full overflow-y-auto overflow-x-hidden', {
	variants: {
		size: {
			default: 'px-2.5 md:px-20',
			compact: 'px-2.5 md:px-6'
		},
		maxWidth: {
			default: 'max-w-screen-xl',
			compact: '',
			article: 'max-w-[65ch]'
		}
	},
	defaultVariants: {
		size: 'default',
		maxWidth: 'default'
	}
});

export const Container = React.forwardRef<HTMLDivElement, TContainerProps>((props, ref) => {
	const { children, tag: Tag = 'div', size, className, ...other } = props;

	return (
		<Tag
			className={cn(
				containerVariants({ size, maxWidth: Tag === 'article' ? 'article' : size, className })
			)}
			ref={ref}
			{...other}
		>
			{children}
		</Tag>
	);
});
Container.displayName = 'Container';

export interface TContainerProps extends React.HTMLAttributes<HTMLDivElement> {
	tag?: 'div' | 'main' | 'article';
	size?: 'default' | 'compact';
}
