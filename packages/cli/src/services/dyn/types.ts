import type { InputPluginOption, MaybePromise, OutputOptions, RollupOptions } from 'rollup';
import type { PackageJson } from 'type-fest';

import type { DynCommand } from '../../DynCommand';
import type { TPath } from '../../utils';

export type Unwrap<T> = T extends Promise<infer U> ? U : T;

export interface TDynConfig {
	figma?: TDynFigmaConfig | TDynFigmaConfigCallback;
	library?: TDynLibraryConfig;
}

// =============================================================================
// Rollup
// =============================================================================

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
}

// =============================================================================
// Library
// =============================================================================

export interface TDynLibraryConfig {
	rollupConfig?: { isBase: boolean; options: TDynRollupOptions };
}

// =============================================================================
// Figma
// =============================================================================

export interface TDynFigmaConfig {
	app?: TDynFigmaAppModuleConfig;
	plugin?: TDynFigmaPluginModuleConfig;
}

export type TDynFigmaConfigCallback = (config: {
	isProduction: boolean;
	isWatchMode: boolean;
}) => Promise<TDynFigmaConfig>;

export type TDynFigmaAppModuleConfig = TDynFigmaBaseModuleConfig & {
	postcssPath?: string;
	rootHtmlPath?: string;
};
export type TDynFigmaPluginModuleConfig = TDynFigmaBaseModuleConfig;

export interface TDynFigmaBaseModuleConfig {
	source?: string;
	output?: string;
	env?: string;
}