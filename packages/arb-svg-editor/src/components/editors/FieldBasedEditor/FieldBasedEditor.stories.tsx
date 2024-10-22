import type { Meta, StoryObj } from '@storybook/react';
import type { TMdtifArtboard } from '@dyn/arb-dtif';
import { LayoutWrapper } from '@dyn/ui';

// eslint-disable-next-line @typescript-eslint/ban-ts-comment -- '@ts-expect-error' doesn't work here
// @ts-ignore Templates live outside the 'src' to not clutter the 'src' folder
import templates from '../../../../.storybook/templates';
import { FieldBasedEditor } from './index';

/**
 * Displays an editor.
 */
const meta = {
	title: 'svg-editor/field-based-editor',
	component: FieldBasedEditor,
	tags: ['autodocs'],
	args: {
		mdtif: templates.mdtif.square as unknown as TMdtifArtboard
	},
	argTypes: {
		mdtif: {
			control: 'select',
			options: Object.keys(templates.mdtif),
			mapping: templates.mdtif
		}
	},
	render: (args) => (
		<LayoutWrapper className="h-screen border-2 border-solid border-red-500" size="full">
			<FieldBasedEditor {...args} />
		</LayoutWrapper>
	)
} satisfies Meta<typeof FieldBasedEditor>;

export default meta;

type Story = StoryObj<typeof meta>;

/**
 * The default editor.
 */
export const Default: Story = {};
