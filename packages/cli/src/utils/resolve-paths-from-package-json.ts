import path from 'node:path';
import type { PackageJson } from 'type-fest';

import type { TPath } from './resolve-paths';

/**
 * Resolves the output path based on the provided export conditions.
 *
 * Handles the format-to-property mapping and default output path.
 * Example: "main": "./dist/cjs/index.js", or "module": "./dist/esm/index.js"
 */
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

	// Try to resolve path of specified format
	const propertyKey = formatToPropertyMap[format];
	const propertyValue: unknown = exportConditions[propertyKey];
	if (typeof propertyValue === 'string') {
		relativeOutputPath = propertyValue;
	}

	// Remove '/index.js' if bundling to dir
	relativeOutputPath = preserveModules
		? relativeOutputPath.replace(/\/[^/]*\.js$/, '')
		: relativeOutputPath;

	return path.resolve(process.cwd(), relativeOutputPath);
}

/**
 * Resolves the input path based on the provided export conditions.
 *
 * Handles the source property for input path.
 * Example: "source": "./src/index.ts"
 */
function resolveInputPathFromPackageJson(exportConditions: PackageJson.ExportConditions): string {
	let relativeInputPath = './src/index.ts';
	const propertyValue: unknown = exportConditions.source;
	if (typeof propertyValue === 'string') {
		relativeInputPath = propertyValue;
	}
	return path.resolve(process.cwd(), relativeInputPath);
}

/**
 * Extracts paths from the package.json based on the export conditions.
 */
export function resolvePathsFromPackageJson(
	packageJson: PackageJson,
	config: TResolveOutputPathFromPackageJsonConfig
): TPath[] {
	const { preserveModules, format } = config;
	const paths: TPath[] = [];
	const exportsArray = Array.isArray(packageJson.exports)
		? packageJson.exports
		: [packageJson.exports];

	for (const exportCondition of exportsArray) {
		// If the export condition is an object (nested conditions or subpaths)
		if (typeof exportCondition === 'object' && exportCondition != null) {
			for (const exportKey of Object.keys(exportCondition)) {
				const nestedExportCondition = exportCondition[exportKey];
				// Handles nested export conditions
				// Example: "package1": { ... }
				if (typeof nestedExportCondition === 'object' && nestedExportCondition != null) {
					paths.push({
						input: resolveInputPathFromPackageJson(
							nestedExportCondition as PackageJson.ExportConditions
						),
						output: resolveOutputPathFromPackageJson(
							nestedExportCondition as PackageJson.ExportConditions,
							{
								preserveModules,
								format
							}
						),
						key: exportKey,
						exportCondition
					});
				}
			}
		} else if (typeof exportCondition === 'string') {
			// TODO: This section needs to be completed to handle export conditions as direct strings
			// Example: "exports": "./main-entry-point.js"
		}
	}

	// If no specific export conditions are found, the code defaults to extracting
	// the 'source', 'main', and 'module' fields from the top level of the package.json
	// Example: "source": "./src/index.ts", "main": "./dist/cjs/index.js", "module": "./dist/esm/index.js"
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
