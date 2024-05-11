import { cva, type VariantProps } from 'class-variance-authority';
import { AlertCircle } from 'lucide-react';
import * as React from 'react';
import { cn } from '@/utils';

const inputVariants = cva(
	'border-input placeholder:text-muted-foreground flex items-center justify-center w-full border bg-transparent px-3 py-1 shadow-sm transition-colors file:border-0 file:bg-transparent file:text-sm file:font-medium focus-visible:outline-none disabled:cursor-not-allowed disabled:opacity-50',
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

export interface TAdvancedInputProps
	extends Omit<React.InputHTMLAttributes<HTMLInputElement>, 'size'>,
		VariantProps<typeof inputVariants> {
	children?: React.ReactElement;
	childrenAfter?: React.ReactElement;
}

const AdvancedInput = React.forwardRef<HTMLInputElement, TAdvancedInputProps>(
	({ className, variant, size, type, children, childrenAfter, ...props }, ref) => {
		if (variant === 'destructive' && childrenAfter == null) {
			// eslint-disable-next-line no-param-reassign -- Ok here
			childrenAfter = (
				<div className="pointer-events-none absolute inset-y-0 right-0 flex items-center pr-3">
					<AlertCircle aria-hidden="true" className="h-5 w-5 text-red-500" />
				</div>
			);
		}

		if (childrenAfter != null || children != null) {
			return (
				<div className="relative">
					{children}
					<input
						className={cn(inputVariants({ variant, size, className }))}
						ref={ref}
						type={type}
						{...props}
					/>
					{childrenAfter}
				</div>
			);
		}

		return (
			<input
				className={cn(inputVariants({ variant, size, className }))}
				ref={ref}
				type={type}
				{...props}
			/>
		);
	}
);
AdvancedInput.displayName = 'AdvancedInput';

export { AdvancedInput };
