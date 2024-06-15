import { Meta, StoryObj } from '@storybook/react';

import { LayoutWrapper } from './LayoutWrapper';

/**
 * Displays a layout wrapper that adjusts padding, margin, and max-width based on the size variant.
 */
const meta = {
	title: 'ui/LayoutWrapper',
	component: LayoutWrapper,
	tags: ['autodocs'],
	argTypes: {
		children: {
			control: 'text'
		},
		size: {
			control: { type: 'select', options: ['default', 'compact', 'full', 'article'] }
		}
	},
	parameters: {
		layout: 'centered'
	},
	args: {
		size: 'default',
		children: <div className="bg-gray-100">This is a layout wrapper content.</div>
	}
} satisfies Meta<typeof LayoutWrapper>;

export default meta;

type Story = StoryObj<typeof meta>;

/**
 * The default layout wrapper with standard padding and max-width.
 */
export const Default: Story = {
	args: {
		size: 'default'
	},
	decorators: [
		(Story) => (
			<div className="border border-gray-300 bg-red-100 p-4">
				<Story />
			</div>
		)
	]
};

/**
 * The compact layout wrapper with reduced max-width.
 */
export const Compact: Story = {
	args: {
		size: 'compact'
	},
	decorators: [
		(Story) => (
			<div className="border border-gray-300 bg-green-100 p-4">
				<Story />
			</div>
		)
	]
};

/**
 * The full layout wrapper with full width and no padding.
 */
export const Full: Story = {
	args: {
		size: 'full'
	},
	decorators: [
		(Story) => (
			<div className="border border-gray-300 bg-blue-100 p-4">
				<Story />
			</div>
		)
	]
};

/**
 * The article layout wrapper with constrained max-width for reading content.
 */
export const Article: Story = {
	args: {
		size: 'article'
	},
	decorators: [
		(Story) => (
			<div className="border border-gray-300 bg-yellow-100 p-4">
				<Story />
			</div>
		)
	]
};
