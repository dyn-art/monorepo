import { cva, type VariantProps } from 'class-variance-authority';
import { AlertCircle } from 'lucide-react';
import * as React from 'react';
import { cn } from '@/utils';

const inputVariants = cva(
	'border-input min-h-[60px] placeholder:text-muted-foreground flex items-center justify-center w-full border bg-transparent px-3 py-2 shadow-sm transition-colors file:border-0 file:bg-transparent file:text-sm file:font-medium focus-visible:outline-none disabled:cursor-not-allowed disabled:opacity-50',
	{
		variants: {
			variant: {
				default: 'focus-visible:ring-ring focus-visible:ring-1',
				destructive: 'text-red-900 ring-2 ring-destructive focus-visible:ring-offset-2'
			},
			size: {
				default: 'h-9 rounded-md text-sm',
				sm: 'h-8 rounded-md text-xs',
				lg: 'h-10 rounded-md text-sm'
			}
		},
		defaultVariants: {
			variant: 'default',
			size: 'default'
		}
	}
);

export interface TAdvancedTextareaProps
	extends Omit<React.TextareaHTMLAttributes<HTMLTextAreaElement>, 'size'>,
		VariantProps<typeof inputVariants> {
	children?: React.ReactElement;
	childrenAfter?: React.ReactElement;
}

const AdvancedTextarea = React.forwardRef<HTMLTextAreaElement, TAdvancedTextareaProps>(
	({ className, variant, size, children, childrenAfter, ...props }, ref) => {
		if (variant === 'destructive' && childrenAfter == null) {
			// eslint-disable-next-line no-param-reassign -- Ok here
			childrenAfter = (
				<div className="pointer-events-none absolute right-0 top-0 flex items-center pr-3 pt-3">
					<AlertCircle aria-hidden="true" className="h-5 w-5 text-red-500" />
				</div>
			);
		}

		if (childrenAfter != null || children != null) {
			return (
				<div className="relative">
					{children}
					<textarea
						className={cn(inputVariants({ variant, size, className }))}
						ref={ref}
						{...props}
					/>
					{childrenAfter}
				</div>
			);
		}

		return (
			<textarea className={cn(inputVariants({ variant, size, className }))} ref={ref} {...props} />
		);
	}
);
AdvancedTextarea.displayName = 'AdvancedTextarea';

export { AdvancedTextarea };
