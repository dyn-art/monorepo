const preserveDirectives = require('rollup-plugin-preserve-directives').default;

/**
 * @type {import('@ibg/cli').TDynConfig}
 */
module.exports = {
	library: {
		rollupConfig: {
			isBase: false,
			options: {
				plugins: [preserveDirectives()] // https://github.com/rollup/rollup/issues/4699
			}
		}
	}
};
