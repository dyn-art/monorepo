import path from 'node:path';
import type { Command } from '@oclif/core';

export async function generateDts(command: Command, options: TGenerateDtsOptions = {}) {
	const { tsConfigPath = path.resolve(process.cwd(), './tsconfig.json') } = options;
	const { execa } = await import('execa');
	command.log('Start generating Typescript Declaration files.', {
		args: [{ tsconfig: tsConfigPath }]
	});
	await execa('pnpm', ['tsc', '--emitDeclarationOnly', '--project', tsConfigPath]);
	command.log('Successfully generated Typescript Declaration files.');
}

export interface TGenerateDtsOptions {
	tsConfigPath?: string;
}
