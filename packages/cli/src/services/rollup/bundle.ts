import chalk from 'chalk';
import {
	rollup,
	type InputPluginOption,
	type OutputOptions,
	type RollupOptions,
	type RollupOutput
} from 'rollup';

import type { DynCommand } from '../../DynCommand';
import { isPlugin } from './is-plugin';

export async function bundleWithRollup(
	command: DynCommand,
	rollupOptions: RollupOptions
): Promise<RollupOutput[]> {
	command.log(
		'🚀 Started bundling.',
		command.isVerbose
			? chalk.gray(
					JSON.stringify({
						args: [
							{
								options: {
									...rollupOptions,
									plugins: pluginsToKeys(rollupOptions.plugins)
								}
							}
						]
					})
			  )
			: ''
	);
	const build = await rollup(rollupOptions);
	const outputs: OutputOptions[] = formatOutput(rollupOptions.output);
	const response = await Promise.all(outputs.map((output) => build.write(output)));
	command.log('🏁 Completed bundling.');
	return response;
}

function pluginsToKeys(plugins: InputPluginOption): string[] {
	const parsedPlugins = Array.isArray(plugins) ? plugins : [plugins];
	return parsedPlugins.map((plugin) => (isPlugin(plugin) ? plugin.name : JSON.stringify(plugin)));
}

function formatOutput(output: RollupOptions['output']): OutputOptions[] {
	if (Array.isArray(output)) {
		return output;
	} else if (output != null) {
		return [output];
	}
	return [];
}
