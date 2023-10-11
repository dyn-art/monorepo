import commonjs from '@rollup/plugin-commonjs';
import esbuild from 'rollup-plugin-esbuild';
import nodeExternals from 'rollup-plugin-node-externals';

import { isExternal as isExternalFactory } from '../is-external-factory';
import { bundleSize, typescriptPaths } from '../plugins';
import type { TDynRollupOptionsCallback } from '../types';

const config: TDynRollupOptionsCallback = async (options) => {
	const { packageJson, path, output, command, tsConfigPath, isProduction } = options;

	return {
		input: path.input,
		output,
		plugins: [
			// Automatically declares NodeJS built-in modules like (node:path, node:fs) as external.
			// This prevents Rollup from trying to bundle these built-in modules,
			// which can cause unresolved dependencies warnings.
			nodeExternals(),
			// Convert CommonJS modules (from node_modules) into ES modules targeted by this app
			commonjs(),
			// Automatically resolve path aliases set in the compilerOptions section of tsconfig.json
			typescriptPaths(command, {
				tsConfigPath,
				shouldResolveRelativeToImporter: false,
				resolveDTsSource: true
			}),
			'import-css', // Plugin placeholder for "rollup-plugin-import-css"
			'wasm', // Plugin placeholder for "rollup-plugin-wasm"
			// Transpile TypeScript code to JavaScript (ES6), and minify in production
			esbuild({
				tsconfig: tsConfigPath,
				minify: isProduction,
				target: 'es6',
				exclude: [/node_modules/],
				loaders: {
					'.json': 'json' // Require @rollup/plugin-commonjs
				},
				sourceMap: false // Configured in rollup 'output' object
			}),
			'copy', // Plugin placeholder for "rollup-plugin-copy"
			// typescript(/* */), // Obsolete as esbuild takes care of configuring typescript
			// babel(/* */), // Obsolete as esbuild takes care of converting ES2015+ modules into compatible JavaScript files
			// terser(/* */), // Obsolete as esbuild takes care of minifying
			await bundleSize(command)
		],
		// Exclude peer dependencies and dependencies from bundle for these reasons:
		// 1. To prevent duplication: If every package included a copy of all its dependencies,
		//    there would be a lot of duplication in node_modules.
		// 2. To enable better versioning: This way, npm can handle installing the latest compatible version.
		// 3. For improved security: If a security vulnerability is found in a dependency,
		//    npm can update it without needing to update this package.
		// 4. Auto Installation: Package managers automatically install these dependencies, so no need to bundle them.
		external: isExternalFactory(packageJson, {
			fileTypesAsExternal: [],
			packageJsonDepsAsExternal: true
		})
	};
};

export default config;
