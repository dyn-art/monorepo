import path from 'node:path';
import type { Command } from '@oclif/core';
import chalk from 'chalk';
import maxmin from 'maxmin';
import type { Plugin } from 'rollup';

function bundleSize(command: Command): Plugin {
	return {
		name: 'rollup-plugin-bundle-size',
		generateBundle(options, bundle) {
			const filePath = options.file;
			if (filePath == null) {
				return;
			}
			const fileName = path.basename(filePath);
			const file = bundle[fileName];
			if (file != null && 'code' in file) {
				const [originalSize, minifiedSize, compressedSize] = maxmin(
					file.code,
					file.code,
					true
				).split(' → ');
				command.log(`Created bundle ${chalk.cyan(fileName)}:`);
				command.log(`- Original Size: ${originalSize}`);
				command.log(`- Minified Size: ${minifiedSize}`);
				command.log(`- Gzipped Size: ${compressedSize}`);
			}
		}
	};
}

export default bundleSize;
