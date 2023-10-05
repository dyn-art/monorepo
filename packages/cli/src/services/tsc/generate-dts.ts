import path from 'node:path';
import type { Command } from '@oclif/core';
import chalk from 'chalk';

export async function generateDts(command: Command, options: TGenerateDtsOptions = {}) {
	const { tsConfigPath = path.resolve(process.cwd(), './tsconfig.json') } = options;
	const { execa } = await import('execa');
	command.log(
		'🚀 Started generating Typescript Declaration files.',
		chalk.gray(
			JSON.stringify({
				args: [{ tsconfig: tsConfigPath }]
			})
		)
	);
	await execa('pnpm', ['tsc', '--emitDeclarationOnly', '--project', tsConfigPath]);
	command.log('🏁 Completed generating Typescript Declaration files.');
}

export interface TGenerateDtsOptions {
	tsConfigPath?: string;
}
