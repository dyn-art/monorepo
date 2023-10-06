// Based on:
// https://github.com/simonhaenisch/rollup-plugin-typescript-paths

import path from 'node:path';
import type { Command } from '@oclif/core';
import chalk from 'chalk';
import type { Plugin } from 'rollup';
import * as ts from 'typescript';

export const typescriptPaths = (
	command: Command,
	options: TTypescriptPathsOptions = {}
): Plugin => {
	const {
		nonRelative = false,
		tsConfigPath = ts.findConfigFile('./', ts.sys.fileExists),
		transform,
		shouldResolveRelativeToImporter = false
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
			const hasMatchingPath = Object.keys(compilerOptionsPaths as object).some((tsPath) =>
				new RegExp(`^${tsPath.replace('*', '.+')}$`).test(importee)
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
			let targetFileName: string = resolvedFileName;
			if (resolvedFileName.endsWith('.d.ts')) {
				if (doesJsFileExist(resolvedFileName)) {
					targetFileName = resolvedFileName.replace(/\.d\.ts$/i, '.js');
				} else if (doesWasmFileExist(resolvedFileName)) {
					targetFileName = resolvedFileName.replace(/(?:\.wasm)?\.d\.ts$/, '.wasm');
				} else {
					return null;
				}
			}

			// Resolve path to target file name
			let resolvedPath: string;
			if (
				(typeof shouldResolveRelativeToImporter === 'boolean' && shouldResolveRelativeToImporter) ||
				(typeof shouldResolveRelativeToImporter === 'function' &&
					shouldResolveRelativeToImporter(targetFileName, importer))
			) {
				resolvedPath = path.relative(path.dirname(importer), targetFileName);
			} else {
				resolvedPath = ts.sys.resolvePath(targetFileName);
			}

			return transform ? transform(resolvedPath) : resolvedPath;
		}
	};
};

function doesJsFileExist(fileName: string): boolean {
	const potentialJsFile = fileName.replace(/\.d\.ts$/, '.js');
	return ts.sys.fileExists(potentialJsFile);
}

function doesWasmFileExist(fileName: string): boolean {
	const potentialWasmFile = fileName.replace(/(?:\.wasm)?\.d\.ts$/, '.wasm');
	return ts.sys.fileExists(potentialWasmFile);
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
	 * Determines if non-relative paths should be resolved based on the tsconfig's `baseUrl`.
	 * The path resolution occurs even if none of the `paths` are matched. By default, this is set to `false`.
	 * Useful in scenarios where the base URL and relative imports need to be considered.
	 *
	 * @see {@link https://www.typescriptlang.org/docs/handbook/module-resolution.html#relative-vs-non-relative-module-imports}
	 * @see {@link https://www.typescriptlang.org/docs/handbook/module-resolution.html#base-url}
	 */
	nonRelative?: boolean;

	/**
	 * Specifies the path to the `tsconfig.json` file.
	 *
	 * @example './path/to/tsconfig.json'
	 */
	tsConfigPath?: string;

	/**
	 * A function that gets invoked when the plugin successfully resolves a path.
	 * It allows for custom transformation of the resolved path before it's returned.
	 *
	 * @param path - The resolved path.
	 * @returns The transformed path string.
	 */
	transform?: (path: string) => string;

	/**
	 * Determines if the resolved paths should be relative to the importing file.
	 */
	shouldResolveRelativeToImporter?:
		| boolean
		| ((source: string, importer: string | undefined) => boolean);
}

type TsConfig = ts.CompilerOptions;

export default typescriptPaths;
