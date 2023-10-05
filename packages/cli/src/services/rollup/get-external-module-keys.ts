import type { ExternalOption } from 'rollup';
import type { PackageJson } from 'type-fest';

export function getExternalModuleKeys(packageJson: PackageJson): ExternalOption {
	const allDeps = {
		...(packageJson.dependencies || {}),
		...(packageJson.peerDependencies || {})
	};
	const externalModuleKeys: string[] = [];
	for (const [key, value] of Object.entries(allDeps)) {
		const isLocalModule = typeof value === 'string' && value.startsWith('link:local_modules/');
		if (!isLocalModule) {
			externalModuleKeys.push(key);
		}
	}
	return externalModuleKeys;
}
