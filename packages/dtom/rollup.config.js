const copy = require('rollup-plugin-copy');

/**
 * @type {import('@dyn/cli').TBaseDynRollupOptions}
 */
module.exports = {
	plugins: [
		copy({
			targets: [{ src: 'rust_modules', dest: 'dist' }]
		})
	]
};
