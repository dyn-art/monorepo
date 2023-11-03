import { Command } from '@oclif/core';
import chalk from 'chalk';

import { promisifyFiglet } from '../utils';

export default class Hello extends Command {
	public async run(): Promise<void> {
		this.log(chalk.yellowBright(await promisifyFiglet('dyn-cli says hello')));
		this.log(`\n`);
	}
}
