import type { Command } from '@oclif/core';
import type { RollupOptions } from 'rollup';

import { compileWithRollup } from './compile';

export async function compileAllWithRollup(command: Command, rollupOptions: RollupOptions[]) {
	return Promise.all(rollupOptions.map((option) => compileWithRollup(command, option)));
}
