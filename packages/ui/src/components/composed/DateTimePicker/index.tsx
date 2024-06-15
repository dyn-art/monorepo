'use client';

import type { CalendarDateTime } from '@internationalized/date';
import { CalendarIcon } from 'lucide-react';
import React from 'react';
import { useDatePickerState } from 'react-stately';
import { Popover, PopoverContent, PopoverTrigger } from '@/components/primitive/layout';

import { Calendar } from '../Calendar';
import { dateToCalendarDateTime } from './date-to-calendar-date-time';
import { DateField } from './DateField';
import { TimeField } from './TimeField';

export const DateTimePicker: React.FC<TProps> = (props) => {
	const { dateTime, onDateTimeUpdate, isDisabled, withTime } = props;
	const contentRef = React.useRef<HTMLDivElement | null>(null);

	const onCalendarSelect = React.useCallback(
		(value?: CalendarDateTime) => {
			const timeZone = Intl.DateTimeFormat().resolvedOptions().timeZone;
			onDateTimeUpdate(value != null ? value.toDate(timeZone) : undefined);
		},
		[onDateTimeUpdate]
	);

	const state = useDatePickerState({
		value: dateTime != null ? dateToCalendarDateTime(dateTime) : undefined,
		onChange: onCalendarSelect,
		isDisabled: props.isDisabled,
		granularity: 'minute'
	});

	return (
		<Popover aria-label="Date Time Picker">
			<DateField
				className="pl-8"
				onChange={state.setValue}
				value={dateTime != null ? dateToCalendarDateTime(dateTime) : undefined}
				withTime={withTime}
			>
				<PopoverTrigger asChild>
					<button
						className="absolute inset-y-0 left-0 flex cursor-pointer items-center pl-3"
						type="button"
					>
						<CalendarIcon className="h-4 w-4" />
					</button>
				</PopoverTrigger>
			</DateField>
			<PopoverContent align="start" className="w-auto" ref={contentRef}>
				<Calendar
					footer={
						<TimeField
							aria-label="Time Picker"
							className="mt-4"
							onChange={state.setTimeValue}
							value={state.timeValue}
						/>
					}
					initialFocus
					mode="single"
					onSelect={(value) => {
						onCalendarSelect(value != null ? dateToCalendarDateTime(value) : undefined);
					}}
					selected={dateTime}
				/>
			</PopoverContent>
		</Popover>
	);
};

interface TProps {
	dateTime?: Date;
	onDateTimeUpdate: (value?: Date) => void;
	withTime?: boolean;
	isDisabled?: boolean;
}
