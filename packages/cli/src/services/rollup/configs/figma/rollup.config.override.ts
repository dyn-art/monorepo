import commonjs from '@rollup/plugin-commonjs';
import { nodeResolve } from '@rollup/plugin-node-resolve';
import replace from '@rollup/plugin-replace';
import esbuild from 'rollup-plugin-esbuild';

import { readDotenvFile } from '../../../../utils';
import type { TBaseDynRollupOptions, TDynRollupOptionsCallbackConfig } from '../../../dyn';
import { bundleSize, typescriptPaths } from '../../plugins';

export async function createOverrideRollupConfig(
	config: TDynBaseRollupOptionsCallbackConfig
): Promise<TBaseDynRollupOptions> {
	const { packageJson, path, output, command, tsConfigPath, isProduction, envPath } = config;
	const env = await readDotenvFile(envPath);

	return {
		input: path.input,
		output,
		plugins: [
			// Resolve and bundle dependencies from node_modules
			nodeResolve({
				extensions: ['.ts', '.tsx', '.js', '.jsx'],
				browser: true
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
				jsxFactory: 'React.createElement',
				jsxFragment: 'React.Fragment',
				sourceMap: false // Configured in rollup 'output' object
			}),
			// typescript(/* */), // Obsolete as esbuild takes care of configuring typescript
			// babel(/* */), // Obsolete as esbuild takes care of converting ES2015+ modules into compatible JavaScript files
			// terser(/* */), // Obsolete as esbuild takes care of minifying
			replace({
				'preventAssignment': true,
				'process.env.npm_package_version': JSON.stringify(packageJson.version),
				'process.env.NODE_ENV': JSON.stringify(isProduction ? 'production' : 'development')
			}),
			replace({
				preventAssignment: true,
				...env
			}),
			await bundleSize(command)
		],
		external: []
	};
}

export type TDynBaseRollupOptionsCallbackConfig = TDynRollupOptionsCallbackConfig & {
	envPath: string;
};
