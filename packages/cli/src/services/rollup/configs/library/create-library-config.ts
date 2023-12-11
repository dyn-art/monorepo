import chalk from 'chalk';
import { defineConfig, type RollupOptions } from 'rollup';
import type { PackageJson } from 'type-fest';
import { toArray } from '@dyn/utils';

import type { DynCommand } from '../../../../DynCommand';
import { resolvePaths, type TPath } from '../../../../utils';
import { mergeRollupConfigs } from '../../merge-rollup-configs';
import { configureCJS, configureESM, type TConfigureModuleConfig } from '../../modules';
import type { TDynRollupOptions, TDynRollupOptionsCallbackConfig } from '../../types';
import { createBaseRollupConfig } from './rollup.config.base';

export async function createLibraryConfig(
	command: DynCommand,
	config: TCreateLibraryConfigConfig
): Promise<RollupOptions[]> {
	const {
		tsConfigPath,
		packageJson,
		format = 'esm',
		rollupOptions = {},
		isProduction = false,
		preserveModules = true,
		sourcemap = true
	} = config;
	const paths = resolvePaths({ paths: config.paths ?? null, packageJson, format, preserveModules });

	command.log(
		`🛣️  Resolved paths from ${chalk.underline('package.json')}'s export conditions${
			command.isVerbose ? `: ${chalk.gray(JSON.stringify(paths))}` : '.'
		}`
	);

	const finalConfigs: RollupOptions[] = [];
	for (const pathItem of paths) {
		const { input: inputPath, output: outputPath } = pathItem;

		// Specific module format configuration
		const moduleConfig: TConfigureModuleConfig = {
			outputPath,
			outputOptions: {
				name: packageJson.name,
				preserveModules,
				sourcemap
			}
		};
		const { output, visualizeFilePath } =
			format === 'esm' ? configureESM(moduleConfig) : configureCJS(moduleConfig);

		// Define rollup config
		const rollupOptionsCallbackConfig: TDynRollupOptionsCallbackConfig = {
			path: {
				input: inputPath,
				output: outputPath
			},
			output,
			tsConfigPath,
			packageJson,
			isProduction,
			command,
			visualizeFilePath
		};

		// Parse base config and override options
		const baseRollupConfig = await createBaseRollupConfig(rollupOptionsCallbackConfig);
		const overrideRollupOptions = toArray(
			typeof rollupOptions === 'object'
				? rollupOptions
				: await rollupOptions(rollupOptionsCallbackConfig)
		);

		// Merge override options into base config
		for (const overrideRollupOption of overrideRollupOptions) {
			finalConfigs.push(
				defineConfig(mergeRollupConfigs(command, baseRollupConfig, overrideRollupOption))
			);
		}
	}

	return finalConfigs;
}

export interface TCreateLibraryConfigConfig {
	tsConfigPath: string;
	packageJson: PackageJson;
	format?: 'cjs' | 'esm';
	paths?: TPath | TPath[];
	isProduction?: boolean;
	preserveModules?: boolean;
	sourcemap?: boolean;
	rollupOptions?: TDynRollupOptions;
}
