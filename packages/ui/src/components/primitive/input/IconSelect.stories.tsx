import { VercelLogoIcon } from '@radix-ui/react-icons';
import type { Meta, StoryObj } from '@storybook/react';
import { CircleIcon, SquareIcon, StarIcon } from 'lucide-react';
import React from 'react';

import { IconSelect, type TIconSelectItem } from './IconSelect';

const ITEMS: Record<string, TIconSelectItem> = {
	rectangle: {
		icon: <SquareIcon className="h-4 w-4" />,
		text: 'Rectangle'
	},
	ellipse: {
		icon: <CircleIcon className="h-4 w-4" />,
		text: 'Ellipse'
	},
	star: {
		icon: <StarIcon className="h-4 w-4" />,
		text: 'Star'
	},
	polygon: {
		icon: <VercelLogoIcon className="h-4 w-4" />,
		text: 'Polygon'
	}
};

/**
 * Displays a list of options for the user to pick from—triggered by a button.
 */
const meta: Meta<typeof IconSelect> = {
	title: 'ui/IconSelect',
	component: IconSelect,
	tags: ['autodocs'],
	argTypes: {},
	parameters: {}
} satisfies Meta<typeof IconSelect>;

export default meta;

type Story = StoryObj<typeof meta>;

/**
 * The default form of the icon select.
 */
export const Default: Story = {
	render: (args) => {
		const [value, setValue] = React.useState<keyof typeof ITEMS>('rectangle');

		return <IconSelect items={ITEMS} onValueChange={setValue} value={value} />;
	}
};
