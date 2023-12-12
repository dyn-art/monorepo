import chalk from 'chalk';
import { defineConfig, type RollupOptions } from 'rollup';
import type { PackageJson } from 'type-fest';

import type { DynCommand } from '../../../../DynCommand';
import { resolvePaths, type TPath } from '../../../../utils';
import { mergeRollupConfigs } from '../../merge-rollup-configs';
import { configureCJS, type TConfigureModuleConfig } from '../../modules';
import type { TBaseDynRollupOptions } from '../../types';
import { createAppRollupConfig } from './rollup.config.app';
import {
	createOverrideRollupConfig,
	type TDynBaseRollupOptionsCallbackConfig
} from './rollup.config.override';
import { createPluginRollupConfig } from './rollup.config.plugin';

export async function createFigmaConfig(
	command: DynCommand,
	config: TCreateFigmaConfigConfig
): Promise<RollupOptions[]> {
	const { tsConfigPath, packageJson, isProduction = false, sourcemap = true } = config;
	const preserveModules = false;
	const paths = resolvePaths({
		paths: config.paths ?? null,
		packageJson,
		format: 'cjs',
		preserveModules
	});

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
		const { output, visualizeFilePath } = configureCJS(moduleConfig);

		// Define rollup config
		const rollupOptionsCallbackConfig: TDynBaseRollupOptionsCallbackConfig = {
			path: {
				input: inputPath,
				output: outputPath
			},
			output,
			tsConfigPath,
			packageJson,
			isProduction,
			command,
			visualizeFilePath,
			envPath: '' // Will be set below
		};

		// Parse base configs and override option
		let baseRollupConfig: TBaseDynRollupOptions;
		if (pathItem.key === 'app') {
			rollupOptionsCallbackConfig.envPath =
				typeof pathItem.exportConditions?.env === 'string'
					? pathItem.exportConditions.env
					: './.env.app';
			baseRollupConfig = await createAppRollupConfig(rollupOptionsCallbackConfig);
		} else if (pathItem.key === 'plugin') {
			rollupOptionsCallbackConfig.envPath =
				typeof pathItem.exportConditions?.env === 'string'
					? pathItem.exportConditions.env
					: './.env.plugin';
			baseRollupConfig = await createPluginRollupConfig(rollupOptionsCallbackConfig);
		} else {
			break;
		}
		const overrideRollupOptions = await createOverrideRollupConfig(rollupOptionsCallbackConfig);

		// Merge override options into base config
		finalConfigs.push(
			defineConfig(mergeRollupConfigs(command, baseRollupConfig, overrideRollupOptions))
		);
	}

	return finalConfigs;
}

export interface TCreateFigmaConfigConfig {
	tsConfigPath: string;
	packageJson: PackageJson;
	paths?: TPath | TPath[];
	isProduction?: boolean;
	sourcemap?: boolean;
}
