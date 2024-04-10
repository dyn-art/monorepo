import React from 'react';
import type { COMP } from '@dyn/dtif-comp';
import type { Composition } from '@dyn/svg-comp';
import {
	CircleIcon,
	CursorArrowIcon,
	IconSelectTrigger,
	Select,
	SelectContent,
	SelectGroup,
	SelectItem,
	SquareIcon,
	StarIcon,
	ToggleGroup,
	ToggleGroupItem,
	VercelLogoIcon
} from '@dyn/ui';

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
				<ToggleGroupItem aria-label="Toggle select" value="Select">
					<CursorArrowIcon className="h-4 w-4" />
				</ToggleGroupItem>
				<ToggleGroupItem aria-label="Toggle shape" className="px-0" value="Shape">
					<Select>
						<IconSelectTrigger className="gap-1 border-none px-2 shadow-none">
							<SquareIcon className="h-4 w-4" />
						</IconSelectTrigger>
						<SelectContent>
							<SelectGroup>
								<SelectItem value="rectangle">
									<div className="flex items-center gap-2">
										<SquareIcon className="h-4 w-4" />
										Rectangle
									</div>
								</SelectItem>
								<SelectItem value="ellipse">
									<div className="flex items-center gap-2">
										<CircleIcon className="h-4 w-4" />
										Circle
									</div>
								</SelectItem>
								<SelectItem value="ellipse">
									<div className="flex items-center gap-2">
										<StarIcon className="h-4 w-4" />
										Star
									</div>
								</SelectItem>
								<SelectItem value="ellipse">
									<div className="flex items-center gap-2">
										<VercelLogoIcon className="h-4 w-4" />
										Polygon
									</div>
								</SelectItem>
							</SelectGroup>
						</SelectContent>
					</Select>
				</ToggleGroupItem>
			</ToggleGroup>
		</aside>
	);
};

interface TProps {
	composition: Composition;
}
