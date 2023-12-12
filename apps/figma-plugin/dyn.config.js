/**
 * @type {import('@dyn/cli').TDynConfig}
 */
module.exports = {
	figma: ({ isWatchMode }) => ({
		app: {
			source: isWatchMode ? './src/dev/app.tsx' : './src/app/index.tsx',
			output: './dist/app.js',
			env: './.env.app',
			postcssPath: './postcss.config.js',
			rootHtmlPath: './index.app.html'
		},
		plugin: {
			source: './src/plugin/index.ts',
			output: './dist/plugin.js',
			env: './.env.plugin'
		}
	})
};
