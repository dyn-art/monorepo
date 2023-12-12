import type { TBaseDynRollupOptions, TDynRollupOptionsCallbackConfig } from '../../../dyn';

export function createPluginRollupConfig(
	config: TDynRollupOptionsCallbackConfig
): TBaseDynRollupOptions {
	const { path, output } = config;

	return {
		input: path.input,
		output,
		plugins: [
			'node-resolve',
			'commonjs',
			'resolve-typescript-paths',
			'esbuild',
			'replace',
			'rollup-plugin-bundle-size'
		],
		external: ['react', 'react-dom']
	};
}
