import path from 'node:path';
import { Command, Flags } from '@oclif/core';
import chalk from 'chalk';
import type { PackageJson } from 'type-fest';

import type { DynRollupOptions } from '../../services';
import { doesFileExist, promisifyFiglet, readJsFile, readJsonFile } from '../../utils';

export default class Bundle extends Command {
	static description = 'Say hello';

	static examples = [];

	static flags = {
		prod: Flags.boolean({
			char: 'p',
			description: 'Production mode',
			required: false,
			default: false
		}),
		bundleStrategy: Flags.string({
			char: 'b',
			description: `Bundle strategy like 'rollup', 'tsc' or 'typesonly'`,
			required: false,
			default: 'rollup',
			options: ['rollup', 'tsc', 'typesonly']
		}),
		analyze: Flags.boolean({
			char: 'a',
			description: 'Analyze bundle',
			required: false,
			default: false
		}),
		sourcemap: Flags.boolean({
			char: 's',
			description: 'Generate sourcemaps',
			required: false,
			default: false
		})
	};

	static args = {};

	public async run(): Promise<void> {
		const { flags } = await this.parse(Bundle);
		const startTime = Date.now();

		// Read in package.json
		const packageJson = await this.getPackageJson();
		if (packageJson == null) {
			this.error(`No package.json file found at '${chalk.underline(process.cwd())}'!`, { exit: 1 });
		}

		// Read in tsconfig.json
		const tsConfigPath = this.getValidTsConfigJsonPath(flags.prod);
		if (tsConfigPath == null) {
			this.error(`No tsconfig.json file found at '${chalk.underline(process.cwd())}'!`, {
				exit: 1
			});
		}

		// Read in rollup.config.js
		const rollupConfig = await this.getRollupConfig();

		this.log(chalk.yellowBright(await promisifyFiglet('dyn bundler')));
		this.log(`\n`);
		this.log(
			`Started bundling package: ${chalk.magenta(
				chalk.underline(packageJson.name ?? 'unknown-package')
			)}.`
		);

		// Bundle package based on bundle strategy
		switch (flags.bundleStrategy) {
			case 'rollup':
				break;
			case 'tsc':
				break;
			case 'typesonly':
				break;
			default:
				this.error(`Unknown build strategy '${flags.buildStrategy}'!`, { exit: 1 });
		}

		this.log(
			`Package was bundled in ${chalk.green(
				chalk.underline(`${((Date.now() - startTime) / 1000).toFixed(2)}s`)
			)}.`
		);
		this.log(`\n`);
		this.exit(1);
	}

	private async getPackageJson(): Promise<PackageJson | null> {
		const packageJsonPath = path.join(process.cwd(), './package.json');
		return readJsonFile<PackageJson>(packageJsonPath);
	}

	private async getRollupConfig(): Promise<DynRollupOptions | null> {
		const rollupConfigPath = path.resolve(process.cwd(), './rollup.config.js');
		const rollupOptions = await readJsFile<DynRollupOptions>(rollupConfigPath);
		if (rollupOptions != null) {
			this.log(`Detected rollup.config at ${chalk.underline(rollupConfigPath)}`);
		}
		return rollupOptions;
	}

	private getValidTsConfigJsonPath(isProduction: boolean): string | null {
		let tsConfigPath: string;
		if (isProduction) {
			tsConfigPath = path.resolve(process.cwd(), './tsconfig.prod.json');
		} else {
			tsConfigPath = path.resolve(process.cwd(), './tsconfig.json');
		}
		if (!doesFileExist(tsConfigPath)) {
			return isProduction ? this.getValidTsConfigJsonPath(false) : null;
		}
		return tsConfigPath;
	}
}
