import type { RollupOptions } from 'rollup';

import type { DynCommand } from '../../DynCommand';
import { watchWithRollup, type TEventWatcher } from './watch';

export async function watchAllWithRollup(
	command: DynCommand,
	rollupOptions: RollupOptions[],
	eventWatcher?: TEventWatcher
): Promise<void> {
	await Promise.all(rollupOptions.map((option) => watchWithRollup(command, option, eventWatcher)));
}
