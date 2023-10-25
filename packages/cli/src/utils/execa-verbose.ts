import type { Command } from '@oclif/core';
import chalk from 'chalk';

export async function execaVerbose(
	toExecuteCommand: string,
	args: string[],
	config: TExecaVerboseConfig
): Promise<void> {
	const { command, cwd, verbose = false } = config;
	const { execa } = await import('execa');
	const subprocess = execa(toExecuteCommand, args, config);

	if (verbose) {
		subprocess.stdout?.on('data', (data) => {
			command.log(chalk.gray(`\t${data}`));
		});

		subprocess.stderr?.on('data', (data) => {
			command.log(chalk.gray(`\t${data}`));
		});
	}

	try {
		await subprocess;
	} catch (error) {
		command.error(
			`An error occured while executing command: ${chalk.yellow(
				`${toExecuteCommand} ${args.join(' ')}`
			)} \n\n ${chalk.gray(`\t${error as any}`)}`
		);
		process.exit(1);
	}
}

interface TExecaVerboseConfig {
	cwd?: string;
	verbose?: boolean;
	command: Command;
}
