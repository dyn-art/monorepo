import type { ExternalOption } from 'rollup';
import type { PackageJson } from 'type-fest';

export function getExternalModuleKeys(packageJson: PackageJson): ExternalOption {
	return Object.keys({
		...(packageJson.dependencies || {}),
		...(packageJson.peerDependencies || {})
	});
}
