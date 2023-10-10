const { wasm } = require('@rollup/plugin-wasm');

/**
 * @type {import('@dyn/cli').TBaseDynRollupOptions}
 */
module.exports = {
	plugins: [
		wasm({
			maxFileSize: 1024 * 1024 * 50 // 50 MB // TODO: Change later when figured out how to load non inline wasm in NextJs
		})
	]
};
