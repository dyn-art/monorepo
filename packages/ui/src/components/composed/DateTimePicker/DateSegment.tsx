import React from 'react';
import { useDateSegment } from 'react-aria';
import type { DateFieldState, DateSegment as TDateSegment } from 'react-stately';
import { cn } from '@/utils';

export const DateSegment: React.FC<TProps> = (props) => {
	const { segment, state } = props;
	const ref = React.useRef<HTMLDivElement>(null);
	const { segmentProps } = useDateSegment(segment, state, ref);

	return (
		<div
			{...segmentProps}
			className={cn(
				'focus:bg-accent focus:text-accent-foreground group box-content flex items-center justify-center rounded-sm px-0.5 text-right tabular-nums outline-none',
				segment.isEditable ? 'text-foreground' : 'text-muted-foreground'
			)}
			ref={ref}
			style={{
				...segmentProps.style,
				minWidth: segment.maxValue ? `${String(segment.maxValue).length}ch` : undefined
			}}
		>
			{segment.isPlaceholder ? (
				<span
					aria-hidden="true"
					className="text-muted-foreground group-focus:text-accent-foreground block w-full text-center italic"
					style={{
						pointerEvents: 'none'
					}}
				>
					{segment.placeholder}
				</span>
			) : (
				segment.text
			)}
		</div>
	);
};

interface TProps {
	segment: TDateSegment;
	state: DateFieldState;
}
