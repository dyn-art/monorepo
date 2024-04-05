import React from 'react';
import type { COMP } from '@dyn/dtif-comp';
import type { Composition } from '@dyn/svg-comp';
import { BoxIcon, CursorArrowIcon, ToggleGroup, ToggleGroupItem } from '@dyn/ui';

import { useInteractionTool } from '../hooks';

export const ToolsBar: React.FC<TProps> = (props) => {
	const { composition } = props;
	const interactionTool = useInteractionTool(composition);

	return (
		<aside className="absolute left-1/2 top-2 flex translate-x-[-50%] flex-col items-center justify-center rounded-lg bg-white px-2 py-2 shadow-md">
			<ToggleGroup
				onValueChange={(value) => {
					let tool: COMP.InteractionTool | null = null;

					switch (value) {
						case 'Select':
							tool = { type: 'Select' };
							break;
						case 'Shape':
							tool = { type: 'Shape', variant: 'Rectangle' };
							break;
						default:
						// do nothing
					}

					if (tool != null) {
						composition.emitInputEvent({
							type: 'Interaction',
							event: { type: 'InteractionToolChanged', tool }
						});
						composition.update();
					}
				}}
				type="single"
				value={interactionTool.type}
			>
				<ToggleGroupItem aria-label="Toggle bold" value="Select">
					<CursorArrowIcon className="h-4 w-4" />
				</ToggleGroupItem>
				<ToggleGroupItem aria-label="Toggle italic" value="Shape">
					<BoxIcon className="h-4 w-4" />
				</ToggleGroupItem>
			</ToggleGroup>
		</aside>
	);
};

interface TProps {
	composition: Composition;
}
