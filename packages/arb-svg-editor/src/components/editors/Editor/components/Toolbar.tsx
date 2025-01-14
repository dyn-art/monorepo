import React from 'react';
import type { ARB } from '@dyn/arb-dtif';
import type { Artboard } from '@dyn/arb-svg-builder';
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
import { useInteractionTool } from '@/hooks';

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

export const Toolbar: React.FC<TProps> = (props) => {
	const { artboard } = props;
	const interactionTool = useInteractionTool(artboard);
	const [shapeToolItem, setShapeToolItem] =
		React.useState<keyof typeof SHAPE_TOOL_ITEMS>('Rectangle');

	return (
		<aside className="absolute left-1/2 top-2 flex translate-x-[-50%] flex-col items-center justify-center rounded-lg bg-white px-2 py-2 shadow-md">
			<ToggleGroup
				onValueChange={(value) => {
					let tool: ARB.InteractionTool | null = null;

					switch (value) {
						case 'Select':
							tool = { type: 'Select' };
							break;
						case 'Shape':
							tool = { type: 'Shape', variant: shapeToolItem as ARB.ShapeVariant };
							break;
						default:
						// do nothing
					}

					if (tool != null) {
						artboard.emitInputEvent('Interaction', { type: 'InteractionToolChanged', tool });
						artboard.update();
					}
				}}
				type="single"
				value={interactionTool.type}
			>
				<ToggleGroupItem aria-label="Toggle select" value="Select">
					<CursorArrowIcon className="h-4 w-4" />
				</ToggleGroupItem>
				<ToggleGroupItem aria-label="Toggle shape" asChild className="px-0" value="Shape">
					{/* Note: Using 'asChild' with `div` because `button` can't be nested which ToogleGroupItem is by default */}
					<div style={{ cursor: 'pointer' }}>
						<IconSelect
							items={SHAPE_TOOL_ITEMS}
							onValueChange={(value) => {
								if (value != shapeToolItem) {
									setShapeToolItem(value);
									artboard.emitInputEvent('Interaction', {
										type: 'InteractionToolChanged',
										tool: { type: 'Shape', variant: value as ARB.ShapeVariant }
									});
								}
							}}
							value={shapeToolItem}
						/>
					</div>
				</ToggleGroupItem>
			</ToggleGroup>
		</aside>
	);
};

interface TProps {
	artboard: Artboard;
}
