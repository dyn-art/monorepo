import { cva, type VariantProps } from 'class-variance-authority';
import * as React from 'react';
import { cn } from '@/utils';

// TODO

const inputVariants = cva(
	'border-input placeholder:text-muted-foreground flex items-center justify-center w-full border bg-transparent px-3 py-1 shadow-sm transition-colors file:border-0 file:bg-transparent file:text-sm file:font-medium focus-visible:outline-none disabled:cursor-not-allowed disabled:opacity-50',
	{
		variants: {
			variant: {
				default: 'focus-visible:ring-ring focus-visible:ring-1',
				destructive: 'ring-2 ring-destructive focus-visible:ring-offset-2'
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

export interface InputProps
	extends Omit<React.InputHTMLAttributes<HTMLInputElement>, 'size'>,
		VariantProps<typeof inputVariants> {
	children?: React.ReactElement;
	childrenAfter?: React.ReactElement;
}

const AdvancedInput = React.forwardRef<HTMLInputElement, InputProps>(
	({ className, variant, size, type, children, childrenAfter, ...props }, ref) => {
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

// const containerVariants = cva(
// 	'placeholder:text-muted-foreground flex items-center justify-center w-full border bg-transparent shadow-sm transition-colors file:bg-transparent file:text-sm file:font-medium disabled:cursor-not-allowed disabled:opacity-50',
// 	{
// 		variants: {
// 			variant: {
// 				default: '',
// 				destructive: ''
// 			},
// 			size: {
// 				default: 'h-9 rounded-md text-sm',
// 				sm: 'h-8 rounded-md text-xs',
// 				lg: 'h-10 rounded-md text-sm'
// 			}
// 		},
// 		defaultVariants: {
// 			variant: 'default',
// 			size: 'default'
// 		}
// 	}
// );

// const inputVariants = cva(
// 	'border-input w-full bg-transparent file:border-0 focus-visible:outline-none',
// 	{
// 		variants: {
// 			variant: {
// 				default: 'focus-visible:ring-ring focus-visible:ring-1',
// 				destructive: 'ring-2 ring-destructive focus-visible:ring-offset-2'
// 			},
// 			size: {
// 				default: 'px-3 py-1',
// 				sm: 'px-2 py-1',
// 				lg: 'px-4 py-2'
// 			}
// 		},
// 		defaultVariants: {
// 			variant: 'default',
// 			size: 'default'
// 		}
// 	}
// );

// export interface InputProps
// 	extends Omit<React.InputHTMLAttributes<HTMLInputElement>, 'size'>,
// 		VariantProps<typeof inputVariants>,
// 		VariantProps<typeof containerVariants> {
// 	children?: React.ReactElement;
// 	childrenAfter?: React.ReactElement;
// }

// const AdvancedInput = React.forwardRef<HTMLInputElement, InputProps>(
// 	({ className, variant, size, type, children, childrenAfter, ...props }, ref) => {
// 		if (childrenAfter != null || children != null) {
// 			return (
// 				<label className={cn(containerVariants({ variant, size, className }))}>
// 					{children}
// 					<input
// 						className={cn(inputVariants({ variant, size, className }))}
// 						ref={ref}
// 						type={type}
// 						{...props}
// 					/>
// 					{childrenAfter}
// 				</label>
// 			);
// 		}

// 		return (
// 			<input
// 				className={cn(
// 					containerVariants({ variant, size, className }),
// 					inputVariants({ variant, size, className })
// 				)}
// 				ref={ref}
// 				type={type}
// 				{...props}
// 			/>
// 		);
// 	}
// );
// AdvancedInput.displayName = 'AdvancedInput';

// export { AdvancedInput };
