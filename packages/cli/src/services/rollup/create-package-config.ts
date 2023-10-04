import path from 'node:path';
import type { Command } from '@oclif/core';
import { defineConfig, type OutputOptions, type RollupOptions } from 'rollup';
import type { PackageJson } from 'type-fest';

import { resolvePathsFromPackageJson } from '../resolve-paths-from-package-json';
import { rollupConfigBase } from './configs';
import { mergeRollupConfigs } from './merge-rollup-configs';
import type { TDynRollupOptions, TDynRollupOptionsCallbackConfig, TPath } from './types';

export async function createRollupPackageConfig(
	command: Command,
	config: TCreatePackageConfigConfig
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

	return Promise.all(
		paths.map(async (pathItem) => {
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
			return defineConfig(
				mergeRollupConfigs(
					await rollupConfigBase(rollupOptionsCallbackConfig),
					typeof rollupOptions === 'object'
						? rollupOptions
						: await rollupOptions(rollupOptionsCallbackConfig),
					{
						command,
						placeholdersInBase: true,
						placeholdersInOverride: true
					}
				)
			);
		})
	);
}

function configureESM(config: TConfigureModuleConfig): TConfigureModuleResponse {
	const { outputOptions, outputPath } = config;
	const preserveModules = outputOptions.preserveModules ?? true;
	return {
		output: {
			...outputOptions,
			...{
				[preserveModules ? 'dir' : 'file']: outputPath,
				format: 'esm',
				preserveModules
			}
		},
		visualizeFilePath: path.resolve(process.cwd(), './.compile/stats-esm.html')
	};
}

function configureCJS(config: TConfigureModuleConfig): TConfigureModuleResponse {
	const { outputOptions, outputPath } = config;
	const preserveModules = outputOptions.preserveModules ?? true;
	return {
		output: {
			...outputOptions,
			...{
				[preserveModules ? 'dir' : 'file']: outputPath,
				format: 'cjs',
				exports: 'named',
				preserveModules
			}
		},
		visualizeFilePath: path.resolve(process.cwd(), './.compile/stats-cjs.html')
	};
}

function resolvePaths(config: {
	paths: TPath | TPath[] | null;
	packageJson: PackageJson;
	format: NonNullable<TCreatePackageConfigConfig['format']>;
	preserveModules: NonNullable<TCreatePackageConfigConfig['preserveModules']>;
}): TPath[] {
	const { paths, packageJson, format, preserveModules } = config;
	const finalPaths: TPath[] = [];
	if (Array.isArray(paths)) {
		finalPaths.push(...paths);
	} else if (typeof paths === 'object' && paths != null) {
		finalPaths.push(paths);
	} else {
		finalPaths.push(...resolvePathsFromPackageJson(packageJson, { format, preserveModules }));
	}
	return finalPaths;
}

export interface TCreatePackageConfigConfig {
	tsConfigPath: string;
	packageJson: PackageJson;
	format?: 'cjs' | 'esm';
	paths?: TPath | TPath[];
	isProduction?: boolean;
	preserveModules?: boolean;
	sourcemap?: boolean;
	rollupOptions?: TDynRollupOptions;
}

interface TConfigureModuleConfig {
	outputPath: string;
	outputOptions: OutputOptions;
}

interface TConfigureModuleResponse {
	output: OutputOptions;
	visualizeFilePath: string;
}
