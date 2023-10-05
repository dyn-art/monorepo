// Based on:
// https://github.com/simonhaenisch/rollup-plugin-typescript-paths

import type { Command } from '@oclif/core';
import chalk from 'chalk';
import type { Plugin } from 'rollup';
import * as ts from 'typescript';

export const typescriptPaths = (
	command: Command,
	options: TTypescriptPathsOptions = {}
): Plugin => {
	const {
		absolute = true,
		nonRelative = false,
		tsConfigPath = ts.findConfigFile('./', ts.sys.fileExists),
		transform
	} = options;
	const compilerOptions = getTsConfig(command, tsConfigPath);

	return {
		name: 'resolve-typescript-paths',
		resolveId: (importee: string, importer?: string) => {
			const compilerOptionsPaths = compilerOptions.paths;
			const isEnabled =
				compilerOptionsPaths != null || (compilerOptions.baseUrl != null && nonRelative);

			// Initial checks for import validity
			if (
				!isEnabled ||
				typeof importer !== 'string' ||
				typeof importee !== 'string' ||
				// Skip processing virtual Rollup modules (those starting with '\0')
				importee.startsWith('\0') ||
				// Can't resolve relative modules, only non-relative
				importee.startsWith('.')
			) {
				return null;
			}

			// Match importee against TypeScript paths to check if it's a valid alias
			const hasMatchingPath = Object.keys(compilerOptionsPaths as object).some((path) =>
				new RegExp(`^${path.replace('*', '.+')}$`).test(importee)
			);

			if (!hasMatchingPath && !nonRelative) {
				return null;
			}

			// Use Typescript's module resolution algorithm to find the correct file represented by the importee
			const { resolvedModule: { resolvedFileName } = {} } = ts.nodeModuleNameResolver(
				importee,
				importer,
				compilerOptions,
				ts.sys
			);
			if (resolvedFileName == null) {
				return;
			}

			// Handle potential .d.ts files and check if corresponding .js exists
			if (resolvedFileName.endsWith('.d.ts') && !doesJsFileExist(resolvedFileName)) {
				return null;
			}

			// Map various TypeScript extensions to their JavaScript equivalents
			const targetFileName = resolvedFileName.replace(/\.d\.ts$|\.ts$|\.tsx$/i, (match) => {
				switch (match) {
					case '.d.ts':
						return '.js';
					case '.tsx':
						return '.jsx';
					case '.ts':
						return '.js';
				}
				command.error(`Unexpected file extension: ${chalk.red(chalk.underline(match))}`, {
					exit: 1
				});
			});

			const resolved = absolute ? ts.sys.resolvePath(targetFileName) : targetFileName;

			return transform ? transform(resolved) : resolved;
		}
	};
};

function doesJsFileExist(fileName: string): boolean {
	const potentialJsFile = fileName.replace(/\.d\.ts$/, '.js');
	return ts.sys.fileExists(potentialJsFile);
}

function getTsConfig(command: Command, configPath?: string): TsConfig {
	const defaultTsConfig: TsConfig = { outDir: '.' };
	if (typeof configPath !== 'string') {
		return defaultTsConfig;
	}

	// Define a host object that implements ParseConfigFileHost.
	// The host provides file system operations and error handling for parsing the configuration file.
	const host: ts.ParseConfigFileHost = {
		fileExists: ts.sys.fileExists,
		readFile: ts.sys.readFile,
		readDirectory: ts.sys.readDirectory,
		useCaseSensitiveFileNames: ts.sys.useCaseSensitiveFileNames,
		getCurrentDirectory: ts.sys.getCurrentDirectory,
		onUnRecoverableConfigFileDiagnostic: (diagnostic) => {
			command.error(
				`Unrecoverable error in config file: ${chalk.red(chalk.underline(diagnostic.messageText))}`,
				{ exit: 1 }
			);
		}
	};

	// Read in tsconfig.json
	const parsedCommandLine = ts.getParsedCommandLineOfConfigFile(configPath, {}, host);

	// Access the parsed tsconfig.json file options
	let resolvedTsConfig = {};
	if (parsedCommandLine != null) {
		resolvedTsConfig = parsedCommandLine.options;
	} else {
		command.error(`Failed to parse TypeScript configuration file: ${chalk.underline(configPath)}`, {
			exit: 1
		});
	}

	return { ...defaultTsConfig, ...resolvedTsConfig };
}

export interface TTypescriptPathsOptions {
	/**
	 * Whether to resolve to absolute paths; defaults to `true`.
	 */
	absolute?: boolean;

	/**
	 * Whether to resolve non-relative paths based on tsconfig's `baseUrl`, even
	 * if none of the `paths` are matched; defaults to `false`.
	 *
	 * https://github.com/simonhaenisch/rollup-plugin-typescript-paths/issues/12
	 *
	 * @see https://www.typescriptlang.org/docs/handbook/module-resolution.html#relative-vs-non-relative-module-imports
	 * @see https://www.typescriptlang.org/docs/handbook/module-resolution.html#base-url
	 */
	nonRelative?: boolean;

	/**
	 * Custom path to your `tsconfig.json`. Use this if the plugin can't seem to
	 * find the correct one by itself.
	 */
	tsConfigPath?: string;

	/**
	 * If the plugin successfully resolves a path, this function allows you to
	 * hook into the process and transform that path before it is returned.
	 */
	transform?: (path: string) => string;
}

type TsConfig = ts.CompilerOptions;

export default typescriptPaths;
