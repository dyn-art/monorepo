import type { NullValue } from 'rollup';
import type { PackageJson } from 'type-fest';

export function isExternal(
	packageJson: PackageJson,
	options: TExternalModuleKeysOptions = {}
): TIsExternal {
	const {
		fileTypesAsExternal = [],
		packageJsonDepsAsExternal = true,
		ignoreRustModules = true
	} = options;
	const allDepKeys = Object.keys({
		...(packageJson.dependencies || {}),
		...(packageJson.peerDependencies || {})
	});
	return (source: string) => {
		let external = false;
		if (packageJsonDepsAsExternal) {
			external = allDepKeys.includes(source);
		}
		if (!external && fileTypesAsExternal.length > 0) {
			external = fileTypesAsExternal.some((fileType) => source.endsWith(fileType));
		}
		if (!external && ignoreRustModules) {
			external = source.includes('/rust_modules/');
		}
		return external;
	};
}

export interface TExternalModuleKeysOptions {
	packageJsonDepsAsExternal?: boolean;
	fileTypesAsExternal?: string[];
	ignoreRustModules?: boolean;
}

export type TIsExternal = (
	source: string,
	importer: string | undefined,
	isResolved: boolean
) => boolean | NullValue;
