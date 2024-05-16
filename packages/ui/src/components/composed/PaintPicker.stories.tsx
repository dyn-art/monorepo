import type { Meta, StoryObj } from '@storybook/react';

import { PaintPicker } from './PaintPicker';

const meta: Meta<typeof PaintPicker> = {
	title: 'ui/PaintPicker',
	component: PaintPicker,
	tags: ['autodocs'],
	argTypes: {},
	render: (args) => (
		<PaintPicker
			{...args}
			onPaintUpdate={() => {
				/* do nothing */
			}}
		/>
	),
	parameters: {
		layout: 'centered'
	},
	args: {
		paint: { type: 'Solid', color: [240, 128, 128] }
	}
} satisfies Meta<typeof PaintPicker>;

export default meta;

type Story = StoryObj<typeof meta>;

/**
 * The default form of the paint picker.
 */
export const Default: Story = {};
