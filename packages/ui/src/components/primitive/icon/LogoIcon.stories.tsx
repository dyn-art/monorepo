// src/stories/LogoIcon.stories.tsx
import type { Meta, StoryObj } from '@storybook/react';

import { LogoIcon } from './LogoIcon';

/**
 * Displays a logo icon component.
 */
const meta = {
	title: 'ui/LogoIcon',
	component: LogoIcon,
	tags: ['autodocs'],
	argTypes: {
		className: {
			control: { type: 'text' },
			defaultValue: 'w-4 h-4'
		}
	},
	parameters: {
		layout: 'centered'
	}
} satisfies Meta<typeof LogoIcon>;

export default meta;

type Story = StoryObj<typeof meta>;

/**
 * The default size of the logo icon.
 */
export const Default: Story = {
	args: {
		className: 'w-4 h-4'
	}
};

/**
 * A smaller size of the logo icon.
 */
export const Small: Story = {
	args: {
		className: 'w-2 h-2'
	}
};

/**
 * A larger size of the logo icon.
 */
export const Large: Story = {
	args: {
		className: 'w-8 h-8'
	}
};
