import path from 'node:path';
import commonjs from '@rollup/plugin-commonjs';
import json from '@rollup/plugin-json';
import { nodeResolve } from '@rollup/plugin-node-resolve';
import esbuild from 'rollup-plugin-esbuild';
import license from 'rollup-plugin-license';

import { getPathDetails } from '../../../../utils';
import type { TBaseDynRollupOptions, TDynRollupOptionsCallbackConfig } from '../../../dyn';
import { bundleSize, typescriptPaths } from '../../plugins';

export async function createBaseRollupConfig(
	config: TDynRollupOptionsCallbackConfig
): Promise<TBaseDynRollupOptions> {
	const { paths, output, command, tsConfigPath, isProduction } = config;
	const pathDetails = getPathDetails(paths.output);

	return {
		input: paths.input,
		output,
		plugins: [
			// Resolve and bundle dependencies from node_modules
			nodeResolve({
				extensions: ['.ts', '.tsx', '.js', '.jsx'],
				browser: false,
				preferBuiltins: true
			}),
			// Convert CommonJS modules (from node_modules) into ES modules targeted by this app
			commonjs(),
			// Automatically resolve path aliases set in the compilerOptions section of tsconfig.json
			typescriptPaths(command, {
				tsConfigPath,
				shouldResolveRelativeToImporter: false,
				resolveDTsSource: true
			}),
			// Transpile TypeScript code to JavaScript (ES6), and minify in production
			esbuild({
				tsconfig: tsConfigPath,
				minify: isProduction,
				target: 'es6',
				exclude: [/node_modules/],
				loaders: {
					'.json': 'json' // Requires @rollup/plugin-commonjs
				},
				sourceMap: false // Configured in rollup 'output' object
			}),
			json(),
			// typescript(/* */), // Obsolete as esbuild takes care of configuring typescript
			// babel(/* */), // Obsolete as esbuild takes care of converting ES2015+ modules into compatible JavaScript files
			// terser(/* */), // Obsolete as esbuild takes care of minifying
			license({
				thirdParty: {
					output: {
						file: path.join(pathDetails.directory, `third-party-licenses.txt`),
						encoding: 'utf-8',
						template(dependencies) {
							return dependencies
								.map(
									(dependency) =>
										`${dependency.license} -- ${dependency.name}:${dependency.version}`
								)
								.join('\n');
						}
					},
					allow: '(Apache-2.0 OR BSD-2-Clause OR BSD-3-Clause OR MIT OR 0BSD)'
				}
			}),
			'copy', // Plugin placeholder for "rollup-plugin-copy"
			await bundleSize(command)
		],
		external: [],
		onwarn(warning, handler) {
			// https://stackoverflow.com/questions/43556940/rollup-js-and-this-keyword-is-equivalent-to-undefined
			if (warning.code === 'THIS_IS_UNDEFINED') {
				return;
			}

			handler(warning);
		}
	};
}
