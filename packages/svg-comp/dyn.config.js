const { wasm } = require('@rollup/plugin-wasm');

/**
 * @type {import('@dyn/cli').TDynConfig}
 */
module.exports = {
	rust: {
		typeDeclarationTargetPaths: ['../dtif-comp/src/_gen/bindings.ts']
	},
	library: {
		rollupConfig: {
			isBase: false,
			options: {
				plugins: [
					// TODO: create two bundles one targeting Node and one the Browser
					// and make it mostly "streamlined" via CLI package

					// Browser
					wasm({
						// Set maxFileSize during development to avoid the overhead
						// of repeatedly copying the WebAssembly (wasm) file to the public folder.
						// When wasm is not embedded as a base64 string in the browser,
						// it is fetched via the fetch api;
						// Hence, it's excluded from the distribution bundle
						// and placed in the public directory of the Remix.
						//
						// Before production build, comment out this
						// setting to reduce the bundle size.
						// Note that inlining wasm as base64 increases size
						// by approximately ≈ 30%, due to encoding overhead,
						// compared to the binary .wasm file.
						maxFileSize: 1024 * 1024 * 50,
						targetEnv: 'browser'
					})
					// Node
					// wasm({
					// 	targetEnv: 'node',
					// 	publicPath: '../../' // esm/package-name/rust_modules -> esm/xyz.wasm
					// })
				]
			}
		}
	}
};
