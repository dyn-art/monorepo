import type { Meta, StoryObj } from '@storybook/react';
import type { TMdtifCanvas } from '@dyn/cnv-dtif';
import { Container } from '@dyn/ui';

// eslint-disable-next-line @typescript-eslint/prefer-ts-expect-error, @typescript-eslint/ban-ts-comment -- '@ts-expect-error' doesn't work here
// @ts-ignore Templates live outside the 'src' to not clutter the 'src' folder
import templates from '../../../.storybook/templates';
import { FieldBasedEditor } from './index';

/**
 * Displays an editor.
 */
const meta = {
	title: 'svg-editor/field-based-editor',
	component: FieldBasedEditor,
	tags: ['autodocs'],
	args: {
		mdtif: templates.mdtif.square as unknown as TMdtifCanvas
	},
	argTypes: {
		mdtif: {
			control: 'select',
			options: Object.keys(templates.mdtif),
			mapping: templates.mdtif
		}
	},
	render: (args) => (
		<Container className="h-screen" size="full" tag="main">
			<FieldBasedEditor {...args} />
		</Container>
	)
} satisfies Meta<typeof FieldBasedEditor>;

export default meta;

type Story = StoryObj<typeof meta>;

/**
 * The default editor.
 */
export const Default: Story = {};
