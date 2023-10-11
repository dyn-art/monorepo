// Based on:
// https://github.com/simonhaenisch/rollup-plugin-typescript-paths

import type { Command } from '@oclif/core';
import type { Plugin } from 'rollup';

import { resolveTsPathsFactory, type TResolveTsPathsFactoryOptions } from '../../../utils';

export const typescriptPaths = (
	command: Command,
	options: TResolveTsPathsFactoryOptions = {}
): Plugin => {
	const resolveTsPaths = resolveTsPathsFactory(command, options);
	return {
		name: 'resolve-typescript-paths',
		resolveId: resolveTsPaths
	};
};

export default typescriptPaths;
