const { wasm } = require('@rollup/plugin-wasm');

/**
 * @type {import('@dyn/cli').TDynConfig}
 */
module.exports = {
	rust: {
		typeDeclarationTargetPaths: ['../comp-dtif/src/_gen/bindings.ts']
	},
	library: {
		rollupConfig: {
			isBase: false,
			options: {
				plugins: [
					wasm({
						// if (isBrowser) {
						// 	customWasmUri = `https://unpkg.com/wasmoon@${version}/dist/glue.wasm`;
						// }
						targetEnv: 'auto-inline'
					})
				]
			}
		}
	}
};
