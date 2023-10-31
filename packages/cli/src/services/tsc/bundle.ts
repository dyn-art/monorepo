import path from 'node:path';
import type { Command } from '@oclif/core';
import chalk from 'chalk';

import { execaVerbose } from '../../utils';

export async function bundleWithTsc(command: Command) {
	const tsconfig = path.resolve(process.cwd(), './tsconfig.json');
	command.log(
		'🚀 Started bundling Typescript files.',
		chalk.gray(
			JSON.stringify({
				args: [{ tsconfig }]
			})
		)
	);
	await execaVerbose('pnpm', ['tsc', '--project', tsconfig], { command });
	command.log('🏁 Completed bundling Typescript files.');
}
