import React from 'react';
import { BoxIcon, CursorArrowIcon, ToggleGroup, ToggleGroupItem } from '@dyn/ui';

export const ToolsBar: React.FC = () => {
	const [value, setValue] = React.useState('cursor');

	return (
		<aside className="absolute left-1/2 top-2 flex translate-x-[-50%] flex-col items-center justify-center rounded-lg bg-white px-2 py-2 shadow-md">
			<ToggleGroup onValueChange={setValue} type="single" value={value}>
				<ToggleGroupItem aria-label="Toggle bold" value="cursor">
					<CursorArrowIcon className="h-4 w-4" />
				</ToggleGroupItem>
				<ToggleGroupItem aria-label="Toggle italic" value="box">
					<BoxIcon className="h-4 w-4" />
				</ToggleGroupItem>
			</ToggleGroup>
		</aside>
	);
};
