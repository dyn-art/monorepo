import React from 'react';
import type { COMP } from '@dyn/dtif-comp';
import type { Composition } from '@dyn/svg-comp';
import {
	CircleIcon,
	CursorArrowIcon,
	IconSelect,
	SquareIcon,
	StarIcon,
	ToggleGroup,
	ToggleGroupItem,
	VercelLogoIcon
} from '@dyn/ui';

import { useInteractionTool } from '../hooks';

const SHAPE_TOOL_ITEMS = {
	Rectangle: {
		icon: <SquareIcon className="h-4 w-4" />,
		text: 'Rectangle'
	},
	Ellipse: {
		icon: <CircleIcon className="h-4 w-4" />,
		text: 'Ellipse'
	},
	Star: {
		icon: <StarIcon className="h-4 w-4" />,
		text: 'Star'
	},
	Polygon: {
		icon: <VercelLogoIcon className="h-4 w-4" />,
		text: 'Polygon'
	}
};

export const ToolsBar: React.FC<TProps> = (props) => {
	const { composition } = props;
	const interactionTool = useInteractionTool(composition);
	const [shapeToolItem, setShapeToolItem] =
		React.useState<keyof typeof SHAPE_TOOL_ITEMS>('Rectangle');

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
							tool = { type: 'Shape', variant: shapeToolItem as COMP.ShapeVariant };
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
					<IconSelect
						items={SHAPE_TOOL_ITEMS}
						onValueChange={(value) => {
							if (value != shapeToolItem) {
								setShapeToolItem(value);
								composition.emitInputEvent({
									type: 'Interaction',
									event: {
										type: 'InteractionToolChanged',
										tool: { type: 'Shape', variant: value as COMP.ShapeVariant }
									}
								});
							}
						}}
						value={shapeToolItem}
					/>
				</ToggleGroupItem>
			</ToggleGroup>
		</aside>
	);
};

interface TProps {
	composition: Composition;
}
