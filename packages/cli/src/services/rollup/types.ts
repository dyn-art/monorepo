import type { Command } from '@oclif/core';
import type { InputPluginOption, MaybePromise, OutputOptions, RollupOptions } from 'rollup';
import type { PackageJson } from 'type-fest';

import type { TPath } from '../../utils';

type Unwrap<T> = T extends Promise<infer U> ? U : T;

export type TDynRollupPlugin = MaybePromise<Unwrap<InputPluginOption> | string>;

export type TBaseDynRollupOptions = Omit<RollupOptions, 'plugins'> & {
	plugins?: TDynRollupPlugin[];
};

export type TDynRollupOptions =
	| TBaseDynRollupOptions
	| TBaseDynRollupOptions[]
	| TDynRollupOptionsCallback;

export type TDynRollupOptionsCallback = (
	config: TDynRollupOptionsCallbackConfig
) => Promise<TBaseDynRollupOptions | TBaseDynRollupOptions[]>;

export interface TDynRollupOptionsCallbackConfig {
	path: TPath;
	output: OutputOptions;
	tsConfigPath: string;
	packageJson: PackageJson;
	isProduction: boolean;
	command: Command;
	visualizeFilePath: string;
}
