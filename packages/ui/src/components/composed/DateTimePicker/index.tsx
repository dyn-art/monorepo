'use client';

import type { CalendarDateTime } from '@internationalized/date';
import { CalendarIcon } from 'lucide-react';
import React from 'react';
import { useDatePickerState, type DatePickerStateOptions } from 'react-stately';
import { Popover, PopoverContent, PopoverTrigger } from '@/components/layout';

import { Calendar } from '../Calendar';
import { dateToCalendarDateTime } from './date-to-calendar-date-time';
import { DateField } from './DateField';
import { TimeField } from './TimeField';

export const DateTimePicker: React.FC<TProps> = (props) => {
	const { dateTime, onDateTimeUpdate, isDisabled } = props;
	const contentRef = React.useRef<HTMLDivElement | null>(null);

	const [open, setOpen] = React.useState(false);

	const onCalendarSelect = React.useCallback(
		(value?: CalendarDateTime) => {
			const timeZone = Intl.DateTimeFormat().resolvedOptions().timeZone;
			onDateTimeUpdate(value != null ? { date: value.toDate(timeZone), hasTime: true } : undefined);
		},
		[onDateTimeUpdate]
	);

	const datePickerProps: DatePickerStateOptions<CalendarDateTime> = {
		value: dateTime != null ? dateToCalendarDateTime(dateTime.date) : undefined,
		onChange: onCalendarSelect,
		isDisabled: props.isDisabled,
		granularity: 'minute'
	};

	const state = useDatePickerState(datePickerProps);

	return (
		<Popover aria-label="Date Time Picker" onOpenChange={setOpen} open={open}>
			<DateField
				className="pl-10"
				onChange={state.setValue}
				value={dateTime != null ? dateToCalendarDateTime(dateTime.date) : undefined}
				withTime={dateTime?.hasTime ?? false}
			>
				<PopoverTrigger asChild>
					<button
						className="absolute inset-y-0 left-0 flex cursor-pointer items-center pl-3 "
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
							onChange={state.setTimeValue}
							value={state.timeValue}
						/>
					}
					initialFocus
					mode="single"
					onSelect={(value) => {
						onCalendarSelect(value != null ? dateToCalendarDateTime(value) : undefined);
					}}
					selected={dateTime?.date}
				/>
			</PopoverContent>
		</Popover>
	);
};

interface TProps {
	dateTime?: { date: Date; hasTime: boolean };
	onDateTimeUpdate: (value?: { date: Date; hasTime: boolean }) => void;
	isDisabled?: boolean;
}
