import React from 'react';

import { GradientPaintInputRow } from './GradientPaintInputRow';
import { ImagePaintInputRow } from './ImagePaintInputRow';
import { SolidPaintInputRow } from './SolidPaintInputRow';
import type { TPaint } from './types';

export const InputRow = React.forwardRef<HTMLDivElement, TProps>((props, forwardedRef) => {
	const { paint, onPaintUpdate, ...popoverTriggerProps } = props;

	// Ensure the popover trigger is always visible by using a wrapper div as the trigger
	// and passing the onClick method to the actual nested trigger.
	// This prevents flickering of the popover when the popover trigger is "switched" (e.g. "Solid" -> "Gradient" popover trigger).
	const { onClick, ...otherPopoverTriggerProps } = popoverTriggerProps;

	return (
		<div {...otherPopoverTriggerProps} ref={forwardedRef}>
			{paint.type === 'Solid' && (
				<SolidPaintInputRow
					onPaintUpdate={onPaintUpdate}
					onPopoverTriggerClick={onClick}
					paint={paint}
				/>
			)}
			{paint.type === 'Gradient' && (
				<GradientPaintInputRow
					onPaintUpdate={onPaintUpdate}
					onPopoverTriggerClick={onClick}
					paint={paint}
				/>
			)}
			{paint.type === 'Image' && (
				<ImagePaintInputRow
					onPaintUpdate={onPaintUpdate}
					onPopoverTriggerClick={onClick}
					paint={paint}
				/>
			)}
		</div>
	);
});
InputRow.displayName = 'InputRow';

interface TProps extends PopoverTriggerProps {
	paint: TPaint;
	onPaintUpdate: (paint: TPaint) => void;
}

// https://www.radix-ui.com/primitives/docs/components/popover
interface PopoverTriggerProps {
	'aria-controls'?: React.HTMLAttributes<HTMLButtonElement>['aria-controls'];
	'aria-expanded'?: React.HTMLAttributes<HTMLButtonElement>['aria-expanded'];
	'aria-haspopup'?: React.HTMLAttributes<HTMLButtonElement>['aria-haspopup'];
	'data-state'?: 'open' | 'closed';
	'onClick'?: React.HTMLAttributes<HTMLButtonElement>['onClick'];
	'type'?: 'button';
}
