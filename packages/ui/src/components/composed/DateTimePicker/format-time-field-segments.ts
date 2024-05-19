import { type DateSegment as TDateSegment } from 'react-stately';

export function formatTimeFieldSegments(segments: TDateSegment[]): TDateSegment[] {
	const hourSegment = segments.find((segment) => segment.type === 'hour');
	const minuteSegment = segments.find((segment) => segment.type === 'minute');
	const periodSegment = segments.find((segment) => segment.type === 'dayPeriod');
	const separatorSegment = segments.find(
		(segment) => segment.type === 'literal' && segment.text === ':'
	);

	if (!hourSegment || !minuteSegment || !separatorSegment) {
		return [];
	}

	const formattedHourSegment = {
		...hourSegment,
		text: hourSegment.text.padStart(2, '0')
	};
	const formattedMinuteSegment = {
		...minuteSegment,
		text: minuteSegment.text.padStart(2, '0')
	};

	const formattedSegments: TDateSegment[] = [
		formattedHourSegment,
		separatorSegment,
		formattedMinuteSegment
	];

	if (periodSegment) {
		formattedSegments.push(periodSegment);
	}

	return formattedSegments;
}
