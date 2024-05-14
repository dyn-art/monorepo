import type { Meta, StoryObj } from '@storybook/react';
import { Mail } from 'lucide-react';

import { AdvancedTextarea } from './AdvancedTextarea';

/**
 * Displays a input or a component that looks like a input.
 */
const meta = {
	title: 'ui/AdvancedTextarea',
	component: AdvancedTextarea,
	tags: ['autodocs'],
	argTypes: {
		children: {
			control: 'text'
		}
	},
	parameters: {
		layout: 'centered'
	},
	args: {
		variant: 'default',
		size: 'default'
	}
} satisfies Meta<typeof AdvancedTextarea>;

export default meta;

type Story = StoryObj<typeof meta>;

/**
 * The default form of the advanced input.
 */
export const Default: Story = {};

/**
 * Use the `destructive` input to indicate errors, alerts, or the need for
 * immediate attention.
 */
export const Destructive: Story = {
	args: {
		variant: 'destructive'
	}
};

/**
 * Add an icon element to a button to enhance visual communication and
 * providing additional context for the action.
 */
export const DestructiveWithIcon: Story = {
	render: (args) => (
		<div>
			<AdvancedTextarea {...args} className="pl-10 pr-10">
				<div className="pointer-events-none absolute left-0 top-0 flex items-center pl-3 pt-3">
					<Mail className="h-4 w-4" />
				</div>
			</AdvancedTextarea>
			<p className="mt-2 text-sm text-red-600" id="email-error">
				Not a valid email address.
			</p>
		</div>
	),
	args: {
		...Destructive.args
	}
};

/**
 * Use the `sm` size for a smaller button, suitable for interfaces needing
 * compact elements without sacrificing usability.
 */
export const Small: Story = {
	args: {
		size: 'sm'
	}
};

/**
 * Use the `lg` size for a larger button, offering better visibility and
 * easier interaction for users.
 */
export const Large: Story = {
	args: {
		size: 'lg'
	}
};

/**
 * Add the `disabled` prop to prevent interactions with the input.
 */
export const Disabled: Story = {
	args: {
		disabled: true
	}
};
