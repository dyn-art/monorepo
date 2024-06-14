const { wasm } = require('@rollup/plugin-wasm');

/**
 * @type {import('@ibg/cli').TDynConfig}
 */
module.exports = {
	rust: {
		typeDeclarationTargetPaths: ['../arb-dtif/src/_gen/bindings.ts']
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
