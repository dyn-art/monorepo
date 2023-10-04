import path from 'node:path';
import type { Command } from '@oclif/core';

export async function compileWithTsc(command: Command) {
	const { execa } = await import('execa');
	const tsconfig = path.resolve(process.cwd(), './tsconfig.json');
	command.log('Start compiling Typescript files.', {
		args: [{ tsconfig }]
	});
	await execa('pnpm', ['tsc', '--project', tsconfig]);
	command.log('Completed compiling Typescript files.');
}
