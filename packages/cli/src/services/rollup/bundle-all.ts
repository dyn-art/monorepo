import type { RollupOptions } from 'rollup';

import type { DynCommand } from '../../DynCommand';
import { bundleWithRollup } from './bundle';

export async function bundleAllWithRollup(
	command: DynCommand,
	rollupOptions: RollupOptions[]
): Promise<void> {
	await Promise.all(rollupOptions.map((option) => bundleWithRollup(command, option)));
}
