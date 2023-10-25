import * as fs from 'node:fs';
import * as path from 'node:path';
import type { Command } from '@oclif/core';
import chalk from 'chalk';
import type { PackageJson } from 'type-fest';

import {
	execaVerbose,
	findNearestTsConfigPath,
	getTsConfigCompilerOptions,
	resolveTsPathsFactory,
	type TResolveTsPaths,
	type TTsConfigCompilerOptions
} from '../../utils';

export async function generateDts(command: Command, options: TGenerateDtsOptions = {}) {
	const {
		tsConfigPath = findNearestTsConfigPath(),
		packageJson,
		shouldResolveTsPaths = true
	} = options;
	command.log(
		'🚀 Started generating Typescript Declaration files.',
		chalk.gray(
			JSON.stringify({
				args: [{ tsconfig: tsConfigPath }]
			})
		)
	);

	// Resolve Typescript compiler options
	if (tsConfigPath == null) {
		command.error('No tsconfig.json found.', { exit: 1 });
	}
	const compilerOptions = getTsConfigCompilerOptions(command, tsConfigPath);

	// Generate declaration files
	await execaVerbose('pnpm', ['tsc', '--emitDeclarationOnly', '--project', tsConfigPath], {
		command
	});

	if (shouldResolveTsPaths && compilerOptions.paths != null) {
		const relativeDeclarationDirPath = getRelativeDeclarationDirPath(compilerOptions, packageJson);
		const declarationFileEnding = '.d.ts';
		const resolveTsPaths = resolveTsPathsFactory(command, {
			compilerOptions: createResolveTsPathsCompilerOptions(
				compilerOptions,
				relativeDeclarationDirPath
			),
			shouldResolveRelativeToImporter: true
		});

		// Update import paths in declaration files
		const declarationFilePaths = getFilePathsWithExtDeep(
			relativeDeclarationDirPath,
			declarationFileEnding
		);
		for (const filePath of declarationFilePaths) {
			let content = fs.readFileSync(filePath, 'utf-8');
			content = updateImportPaths(content, filePath, resolveTsPaths);
			content = updateExportPaths(content, filePath, resolveTsPaths);
			fs.writeFileSync(filePath, content);
		}
	}

	command.log('🏁 Completed generating Typescript Declaration files.');
}

function getRelativeDeclarationDirPath(
	compilerOptions: TTsConfigCompilerOptions,
	packageJson?: PackageJson
): string {
	if (compilerOptions.declarationDir) {
		return path.relative(process.cwd(), compilerOptions.declarationDir);
	}
	return packageJson?.types ?? './dist/types';
}

function createResolveTsPathsCompilerOptions(
	compilerOptions: TTsConfigCompilerOptions,
	relativeDeclarationDirPath: string
) {
	const basePath = path.resolve(compilerOptions.pathsBasePath?.toString() ?? process.cwd());
	const relativeRootDir = path.relative(basePath, compilerOptions.rootDir ?? './src');

	// Create compiler options
	const updatedPaths = compilerOptions.paths
		? Object.fromEntries(
				Object.entries(compilerOptions.paths).map(([key, value]) => [
					key,
					value.map((tsPath: string) =>
						tsPath.replace(
							new RegExp(`^\\.${path.sep}${relativeRootDir}`),
							`.${path.sep}${relativeDeclarationDirPath}`
						)
					)
				])
		  )
		: undefined;

	return {
		...compilerOptions,
		rootDir: relativeDeclarationDirPath,
		paths: updatedPaths,
		outDir: undefined,
		declarationDir: undefined
	};
}

function updateImportPaths(
	content: string,
	filePath: string,
	resolveTsPath: TResolveTsPaths
): string {
	// Regular expression to match import statements with named groups
	const importRegex =
		/(?:^|\n)import\s+(?<imported>[^\n]*)\s+from\s+(?<quote>'|")(?<path>[^'"]+)(?:\2);?/;

	return content.replace(importRegex, (match, imported, quote, importPath) => {
		if (
			typeof imported !== 'string' ||
			typeof quote !== 'string' ||
			typeof importPath !== 'string'
		) {
			return match;
		}
		const resolvedPath = resolveTsPath(importPath, filePath);
		if (resolvedPath) {
			return `import type ${imported} from ${quote}${resolvedPath.replace(
				'/index.d.ts',
				''
			)}${quote};`;
		}
		return match;
	});
}

function updateExportPaths(
	content: string,
	filePath: string,
	resolveTsPath: TResolveTsPaths
): string {
	// Regular expression to match export statements with named groups
	const exportRegex =
		/(?:^|\n)export\s+(?<exported>\*|\{[^}]*\})\s+from\s+(?<quote>'|")(?<path>[^'"]+)(?:\2);?/;

	return content.replace(exportRegex, (match, exported, quote, exportPath) => {
		if (
			typeof exported !== 'string' ||
			typeof quote !== 'string' ||
			typeof exportPath !== 'string'
		) {
			return match;
		}
		const resolvedPath = resolveTsPath(exportPath, filePath);
		if (resolvedPath) {
			return `export ${exported} from ${quote}${resolvedPath.replace('/index.d.ts', '')}${quote};`;
		}
		return match;
	});
}

function getFilePathsWithExtDeep(dir: string, ext: string): string[] {
	let result: string[] = [];
	const fileNames = fs.readdirSync(dir);
	for (const fileName of fileNames) {
		const fullPath = path.join(dir, fileName);
		const stat = fs.statSync(fullPath);
		if (stat.isDirectory()) {
			result = result.concat(getFilePathsWithExtDeep(fullPath, ext));
		} else if (fileName.endsWith(ext)) {
			result.push(fullPath);
		}
	}
	return result;
}

export interface TGenerateDtsOptions {
	tsConfigPath?: string;
	packageJson?: PackageJson;
	shouldResolveTsPaths?: boolean;
}
