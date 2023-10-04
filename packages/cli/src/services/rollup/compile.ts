import type { Command } from '@oclif/core';
import { rollup, type InputPluginOption, type OutputOptions, type RollupOptions } from 'rollup';

import { isPlugin } from './is-plugin';

export async function compileWithRollup(command: Command, rollupOptions: RollupOptions) {
	command.log('Start compiling.', {
		args: [
			{
				options: {
					...rollupOptions,
					plugins: pluginsToKeys(rollupOptions.plugins)
				}
			}
		]
	});
	const build = await rollup(rollupOptions);
	const outputs: OutputOptions[] = formatOutput(rollupOptions.output);
	const response = await Promise.all(outputs.map((output) => build.write(output)));
	command.log('Completed compiling.');
	return response;
}

function pluginsToKeys(plugins: InputPluginOption) {
	return Array.isArray(plugins)
		? plugins.map((plugin) => (isPlugin(plugin) ? plugin.name : plugin))
		: plugins;
}

function formatOutput(output: RollupOptions['output']): OutputOptions[] {
	if (Array.isArray(output)) {
		return output;
	} else if (output != null) {
		return [output];
	}
	return [];
}
