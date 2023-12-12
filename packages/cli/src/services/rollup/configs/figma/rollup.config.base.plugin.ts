import type { TBaseDynRollupOptions, TDynRollupOptionsCallbackConfig } from '../../../dyn';

export async function createPluginRollupConfig(
	config: TDynRollupOptionsCallbackConfig
): Promise<TBaseDynRollupOptions> {
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
