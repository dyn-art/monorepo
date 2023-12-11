import type { PackageJson } from 'type-fest';

import { resolvePathsFromPackageJson } from './resolve-paths-from-package-json';

export function resolvePaths(config: {
	paths: TPath | TPath[] | null;
	packageJson: PackageJson;
	format: 'cjs' | 'esm';
	preserveModules: boolean;
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

export interface TPath {
	output: string;
	input: string;
	key?: string;
	exportCondition?: PackageJson.ExportConditions;
}
