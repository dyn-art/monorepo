const { wasm } = require('@rollup/plugin-wasm');

/**
 * @type {import('@dyn/cli').TBaseDynRollupOptions}
 */
module.exports = {
	plugins: [wasm()]
};
