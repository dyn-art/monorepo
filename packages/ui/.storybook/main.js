import { dirname, join } from 'path';
import { mergeConfig } from 'vite';
import tsconfigPaths from 'vite-tsconfig-paths';

/**
 * This function is used to resolve the absolute path of a package.
 * It is needed in projects that use Yarn PnP or are set up within a monorepo.
 *
 * https://storybook.js.org/docs/faq#how-do-i-fix-module-resolution-in-special-environments
 */
function getAbsolutePath(value) {
	return dirname(require.resolve(join(value, 'package.json')));
}

/** @type { import('@storybook/react-vite').StorybookConfig } */
const config = {
	stories: ['../src/**/*.mdx', '../src/**/*.stories.@(js|jsx|mjs|ts|tsx)'],
	addons: [
		getAbsolutePath('@storybook/addon-links'),
		getAbsolutePath('@storybook/addon-essentials'),
		getAbsolutePath('@storybook/addon-interactions')
	],
	framework: {
		name: getAbsolutePath('@storybook/react-vite'),
		options: {}
	},
	docs: {
		autodocs: 'tag'
	},
	async viteFinal(config) {
		return mergeConfig(config, {
			plugins: [tsconfigPaths()]
		});
	}
};

export default config;
