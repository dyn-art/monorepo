import React from 'react';
import { BoxIcon, Button, CursorArrowIcon } from '@dyn/ui';

// TODO:
// 1. Add shdcn resizable component: https://ui.shadcn.com/docs/components/resizable
// 2. Integrate resizable component with editor
// 3. Figure out how to get pixel size of resizable component (https://react-resizable-panels.vercel.app/examples/imperative-panel-api)
// 4. Extract Canvas as separate component not as child of Editor Component
// 5. Remove width and height constraints for Editor and instead use resizable size for Canvas width and height
// 6. Complete ToolsBar
// 7.

export const ToolsBar: React.FC = () => {
	return (
		<aside className="absolute left-1/2 top-0 flex translate-x--1/2 flex-col items-center justify-center rounded-lg bg-red-100 p-2 px-4 py-2">
			<ul className="relative flex items-center space-x-4">
				<li className="relative">
					<Button className="rounded-md bg-[#444] text-white" size="icon">
						<CursorArrowIcon className="text-white-400" />
					</Button>
				</li>
				<li className="relative">
					<Button className="rounded-md bg-[#444] text-white" size="icon">
						<BoxIcon className="text-white" />
					</Button>
				</li>
			</ul>
		</aside>
	);
};
