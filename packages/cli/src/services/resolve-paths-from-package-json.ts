import path from 'node:path';
import type { PackageJson } from 'type-fest';

import type { TPath } from './rollup/types';

function resolveOutputPathFromPackageJson(
	exportConditions: PackageJson.ExportConditions,
	config: TResolveOutputPathFromPackageJsonConfig
): string {
	const { format, preserveModules } = config;
	let relativeOutputPath = `./dist/${format}/index.js`;
	const formatToPropertyMap = {
		esm: 'module',
		cjs: 'main'
	};
	const propertyKey = formatToPropertyMap[format];
	const propertyValue = exportConditions[propertyKey];
	if (typeof propertyValue === 'string') {
		relativeOutputPath = propertyValue;
	}
	relativeOutputPath = preserveModules
		? relativeOutputPath.replace(/\/[^/]*\.js$/, '') // remove '/index.js' if bundling to dir
		: relativeOutputPath;
	return path.resolve(process.cwd(), relativeOutputPath);
}

function resolveInputPathFromPackageJson(exportConditions: PackageJson.ExportConditions): string {
	let relativeInputPath = './src/index.ts';
	const propertyValue = exportConditions.source;
	if (typeof propertyValue === 'string') {
		relativeInputPath = propertyValue;
	}
	return path.resolve(process.cwd(), relativeInputPath);
}

export function resolvePathsFromPackageJson(
	packageJson: PackageJson,
	config: TResolveOutputPathFromPackageJsonConfig
): TPath[] {
	const { preserveModules, format } = config;
	const paths: TPath[] = [];

	const packageJsonExports = Array.isArray(packageJson.exports)
		? packageJson.exports
		: [packageJson.exports];
	for (const exportCondition of packageJsonExports) {
		if (typeof exportCondition === 'object' && exportCondition != null) {
			paths.push({
				input: resolveInputPathFromPackageJson(exportCondition),
				output: resolveOutputPathFromPackageJson(exportCondition, { preserveModules, format })
			});
		} else if (typeof exportCondition === 'string') {
			// TODO: ERROR
		}
	}

	if (paths.length === 0) {
		paths.push({
			input: resolveInputPathFromPackageJson(packageJson as PackageJson.ExportConditions),
			output: resolveOutputPathFromPackageJson(packageJson as PackageJson.ExportConditions, {
				preserveModules,
				format
			})
		});
	}

	return paths;
}

interface TResolveOutputPathFromPackageJsonConfig {
	format: 'esm' | 'cjs';
	preserveModules: boolean;
}
