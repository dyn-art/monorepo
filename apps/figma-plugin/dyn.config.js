/**
 * @type {import('@dyn/cli').TDynConfig}
 */
module.exports = {
	figma: ({ isProduction }) => ({
		app: {
			source: isProduction ? './src/app/index.tsx' : './dev/app/index.tsx',
			output: './dist/app.js',
			env: './.env.app',
			postcssPath: './postcss.config.js',
			rootHtmlPath: isProduction ? './index.prod.html' : './index.html'
		},
		plugin: {
			source: isProduction ? './src/plugin/index.ts' : './src/plugin/index.ts',
			output: './dist/plugin.js',
			env: './.env.plugin'
		}
	})
};
