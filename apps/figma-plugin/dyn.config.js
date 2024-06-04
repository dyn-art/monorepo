/**
 * @type {import('@ibg/cli').TDynConfig}
 */
module.exports = {
	figma: ({ isWatchMode }) => ({
		app: {
			source: isWatchMode ? './src/dev/app.tsx' : './src/app/index.tsx',
			output: './dist/app.js',
			env: './.env.app',
			postcssConfigPath: './postcss.config.js'
			// htmlTemplatePath: './index.app.html'
		},
		plugin: {
			source: './src/plugin/index.ts',
			output: './dist/plugin.js',
			env: './.env.plugin'
		}
	})
};
