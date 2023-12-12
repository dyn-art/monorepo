import type { InputPluginOption, MaybePromise, OutputOptions, RollupOptions } from 'rollup';
import type { PackageJson } from 'type-fest';
import type { Unwrap } from '@dyn/types/utility';

import type { DynCommand } from '../../DynCommand';
import type { TPath } from '../../utils';

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
	command: DynCommand;
	visualizeFilePath: string;
}
