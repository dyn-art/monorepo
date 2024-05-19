import { CalendarDateTime } from '@internationalized/date';

export function dateToCalendarDateTime(date: Date): CalendarDateTime {
	return new CalendarDateTime(
		date.getFullYear(),
		date.getMonth() + 1, // JavaScript months are 0-based
		date.getDate(),
		date.getHours(),
		date.getMinutes(),
		date.getSeconds(),
		date.getMilliseconds()
	);
}
