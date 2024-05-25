'use client';

import { createCalendar } from '@internationalized/date';
import React from 'react';
import { useDateField, useLocale, type AriaDateFieldProps, type DateValue } from 'react-aria';
import { useDateFieldState } from 'react-stately';
import { cn } from '@/utils';

import { DateSegment } from './DateSegment';
import { formatDateFieldSegments } from './format-date-field-segments';

export const DateField: React.FC<TProps> = (props) => {
	const { withTime, children, childrenAfter, className, ...dateFieldProps } = props;
	const ref = React.useRef<HTMLDivElement | null>(null);
	const { locale } = useLocale();
	const state = useDateFieldState({
		...dateFieldProps,
		locale,
		createCalendar
	});
	const { fieldProps } = useDateField(dateFieldProps, state, ref);
	const segments = React.useMemo(
		() => formatDateFieldSegments(state.segments, withTime),
		[state.segments, withTime]
	);

	return (
		<div className="relative">
			{children}
			<div
				{...fieldProps}
				className={cn(
					'border-input placeholder:text-muted-foreground focus-visible:ring-ring flex h-9 w-full rounded-md border bg-transparent px-3 py-1 text-sm shadow-sm transition-colors file:border-0 file:bg-transparent file:text-sm file:font-medium focus-visible:outline-none focus-visible:ring-1 disabled:cursor-not-allowed disabled:opacity-50',
					dateFieldProps.isDisabled ? 'cursor-not-allowed opacity-50' : '',
					className
				)}
				ref={ref}
			>
				{segments.map((segment, i) => (
					<DateSegment key={i} segment={segment} state={state} />
				))}
			</div>
			{childrenAfter}
		</div>
	);
};

interface TProps extends AriaDateFieldProps<DateValue> {
	withTime?: boolean;
	children?: React.ReactElement;
	childrenAfter?: React.ReactElement;
	className?: string;
}
