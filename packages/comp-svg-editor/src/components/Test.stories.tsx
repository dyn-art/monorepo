import type { Meta, StoryObj } from '@storybook/react';

import { Test } from './Test';

// TODO: Error https://github.com/storybookjs/storybook/discussions

/**
 * Displays a button or a component that looks like a button.
 */
const meta = {
	title: 'ui/Test',
	component: Test,
	tags: ['autodocs']
} satisfies Meta<typeof Test>;

export default meta;

type Story = StoryObj<typeof meta>;

/**
 * The default form of the button, used for primary actions and commands.
 */
export const Default: Story = {};
