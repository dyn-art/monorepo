import type { Meta, StoryObj } from '@storybook/react';
import type { ARB } from '@dyn/arb-dtif';
import { LayoutWrapper } from '@dyn/ui';

// eslint-disable-next-line @typescript-eslint/ban-ts-comment -- '@ts-expect-error' doesn't work here
// @ts-ignore Templates live outside the 'src' to not clutter the 'src' folder
import templates from '../../../../.storybook/templates';
import { CanvaStyleEditor } from './index';

/**
 * Displays an editor.
 */
const meta = {
	title: 'svg-editor/canva-style-editor',
	component: CanvaStyleEditor,
	tags: ['autodocs'],
	args: {
		dtif: templates.dtif.default as unknown as ARB.DtifArtboard
	},
	argTypes: {
		dtif: {
			control: 'select',
			options: Object.keys(templates.dtif),
			mapping: templates.dtif
		}
	},
	render: (args) => (
		<LayoutWrapper className="h-screen border-2 border-solid border-red-500" size="full">
			<CanvaStyleEditor {...args} />
		</LayoutWrapper>
	)
} satisfies Meta<typeof CanvaStyleEditor>;

export default meta;

type Story = StoryObj<typeof meta>;

/**
 * The default editor.
 */
export const Default: Story = {};
