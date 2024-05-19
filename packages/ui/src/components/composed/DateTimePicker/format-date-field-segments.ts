import { type DateSegment as TDateSegment } from 'react-stately';

import { formatTimeFieldSegments } from './format-time-field-segments';

export function formatDateFieldSegments(
	segments: TDateSegment[],
	withTime = false
): TDateSegment[] {
	const daySegment = segments.find((segment) => segment.type === 'day');
	const monthSegment = segments.find((segment) => segment.type === 'month');
	const yearSegment = segments.find((segment) => segment.type === 'year');
	const separatorSegment = segments.find((segment) => segment.type === 'literal');

	if (!daySegment || !monthSegment || !yearSegment || !separatorSegment) {
		return [];
	}

	const formattedDaySegment = {
		...daySegment,
		text: daySegment.text.padStart(2, '0')
	};
	const formattedMonthSegment = {
		...monthSegment,
		text: monthSegment.text.padStart(2, '0')
	};
	const formattedSeparatorSegment = { ...separatorSegment, text: '/' };

	return [
		formattedMonthSegment,
		formattedSeparatorSegment,
		formattedDaySegment,
		formattedSeparatorSegment,
		yearSegment,
		...(withTime ? [{ ...separatorSegment, text: ', ' }, ...formatTimeFieldSegments(segments)] : [])
	];
}
