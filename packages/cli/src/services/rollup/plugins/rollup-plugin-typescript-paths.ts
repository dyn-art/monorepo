// Based on:
// https://github.com/simonhaenisch/rollup-plugin-typescript-paths

import type { Plugin } from 'rollup';

import type { DynCommand } from '../../../DynCommand';
import { resolveTsPathsFactory, type TResolveTsPathsFactoryOptions } from '../../../utils';

export const typescriptPaths = (
	command: DynCommand,
	options: TResolveTsPathsFactoryOptions = {}
): Plugin => {
	const resolveTsPaths = resolveTsPathsFactory(command, options);
	return {
		name: 'resolve-typescript-paths',
		resolveId: resolveTsPaths
	};
};

export default typescriptPaths;
