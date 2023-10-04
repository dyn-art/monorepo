import type { Command } from '@oclif/core';
import type { RollupOptions } from 'rollup';

import { bundleWithRollup } from './bundle';

export async function bundleAllWithRollup(command: Command, rollupOptions: RollupOptions[]) {
	return Promise.all(rollupOptions.map((option) => bundleWithRollup(command, option)));
}
