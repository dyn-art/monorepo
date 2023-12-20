import React from 'react';
import {
	Button,
	CheckIcon,
	CopyIcon,
	Tooltip,
	TooltipContent,
	TooltipProvider,
	TooltipTrigger
} from '@dyn/ui';

import { copyToClipboard } from '../../core/utils';

export const ClipboardButton: React.FC<TProps> = (props) => {
	const { toCopy, className } = props;
	const [isClipboardButtonPressed, setIsClipboardButtonPressed] = React.useState(false);

	const copyText = React.useCallback((toCopyText: string) => {
		setIsClipboardButtonPressed(true);
		copyToClipboard(toCopyText).catch(() => {
			// do nothing
		});
		setTimeout(() => {
			setIsClipboardButtonPressed(false);
		}, 2000);
	}, []);

	const tooltipText = isClipboardButtonPressed ? 'copied' : 'copy';

	return (
		<TooltipProvider>
			<Tooltip>
				<TooltipTrigger asChild>
					<Button
						variant="secondary"
						size="icon"
						className={className}
						onClick={(event) => {
							event.preventDefault();
							event.stopPropagation();
							copyText(toCopy);
						}}
					>
						{isClipboardButtonPressed ? <CheckIcon /> : <CopyIcon />}
					</Button>
				</TooltipTrigger>
				<TooltipContent side="bottom">
					<p>{tooltipText}</p>
				</TooltipContent>
			</Tooltip>
		</TooltipProvider>
	);
};

interface TProps {
	toCopy: string;
	className?: string;
}
