import type { PackageJson } from 'type-fest';

import { resolvePathsFromPackageJson } from './resolve-paths-from-package-json';
import { toArray } from './to-array';

export function resolvePaths(config: {
	paths: TPath | TPath[] | null;
	packageJson: PackageJson;
	format: 'cjs' | 'esm' | 'types';
	preserveModules: boolean;
}): TPath[] {
	const { paths, packageJson, format, preserveModules } = config;
	const finalPaths: TPath[] = [];

	if (paths != null) {
		finalPaths.push(...toArray(paths));
	} else {
		finalPaths.push(...resolvePathsFromPackageJson(packageJson, { format, preserveModules }));
	}

	return finalPaths;
}

export interface TPath {
	output: string;
	input: string;
	key?: string;
}
