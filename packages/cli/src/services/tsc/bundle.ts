import path from 'node:path';
import type { Command } from '@oclif/core';
import chalk from 'chalk';

export async function bundleWithTsc(command: Command) {
	const { execa } = await import('execa');
	const tsconfig = path.resolve(process.cwd(), './tsconfig.json');
	command.log(
		'🚀 Started bundling Typescript files.',
		chalk.gray(
			JSON.stringify({
				args: [{ tsconfig }]
			})
		)
	);
	await execa('pnpm', ['tsc', '--project', tsconfig]);
	command.log('🏁 Completed bundling Typescript files.');
}
