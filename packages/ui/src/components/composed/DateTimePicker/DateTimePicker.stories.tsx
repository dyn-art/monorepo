import { getLocalTimeZone, now } from '@internationalized/date';
import type { Meta, StoryObj } from '@storybook/react';
import React from 'react';

import { DateField } from './DateField';
import { DateTimePicker } from './index';
import { TimeField } from './TimeField';

const meta: Meta<typeof DateTimePicker> = {
	title: 'ui/DateTimePicker',
	component: DateTimePicker,
	tags: ['autodocs'],
	argTypes: {},
	render: (args) => {
		const [date, setDate] = React.useState<Date | undefined>(new Date());

		return <DateTimePicker {...args} dateTime={date} onDateTimeUpdate={setDate} withTime />;
	},
	parameters: {
		layout: 'centered'
	},
	args: {}
} satisfies Meta<typeof DateTimePicker>;

export default meta;

type Story = StoryObj<typeof meta>;

/**
 * The default form of the paint picker.
 */
export const Default: Story = {};

export const TimeFieldDefault: Story = {
	render: (args) => <TimeField />,
	args: {}
};

export const DateFieldDefault: Story = {
	render: (args) => <DateField />,
	args: {}
};

export const DateFieldWithTime: Story = {
	render: (args) => <DateField placeholderValue={now(getLocalTimeZone())} withTime />,
	args: {}
};
