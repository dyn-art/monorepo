import type { Command } from '@oclif/core';
import { mergeWith } from 'lodash';
import type { InputPluginOption, Plugin, RollupOptions } from 'rollup';

import type { TBaseDynRollupOptions, TDynRollupPlugin } from '.';
import { isPlugin } from './is-plugin';

/**
 * Merges two Rollup configurations with special handling for plugins.
 *
 * @param baseConfig - The primary/base configuration to start with.
 * @param overrideConfig - The configuration that provides overriding values.
 * @param config - Settings for controlling placeholder behavior.
 * @returns - The merged configuration.
 */
export function mergeRollupConfigs(
	baseConfig: TBaseDynRollupOptions,
	overrideConfig: TBaseDynRollupOptions,
	config: TMergeRollupConfigsConfig
): RollupOptions {
	return mergeWith({}, baseConfig, overrideConfig, (objValue, srcValue, key) => {
		if (key === 'plugins') {
			return mergePlugins(objValue as Plugin[], srcValue as Plugin[], config);
		}
	}) as RollupOptions;
}

/**
 * Merge the plugins of two configurations.
 *
 * This function merges two plugin arrays. If placeholders are used, they will be replaced
 * with the actual plugin instances from the other array.
 *
 * @param basePlugins - Plugin list from the base configuration.
 * @param overridePlugins - Plugin list from the overriding configuration.
 * @param options - Settings for controlling placeholder behavior.
 * @returns - The merged plugin list.
 */
function mergePlugins(
	basePlugins: TDynRollupPlugin[],
	overridePlugins: TDynRollupPlugin[],
	config: TMergeRollupConfigsConfig
): InputPluginOption {
	const { command, placeholdersInBase = true, placeholdersInOverride = true } = config;

	// Create a map to store plugin instances by name.
	const allPluginsMap: Record<string, Plugin> = {};

	// Helper function to collect plugin instances into the map.
	const gatherPlugins = (plugin: TDynRollupPlugin) => {
		// We only care about Plugin objects with a name property
		if (typeof plugin === 'object' && plugin != null && 'name' in plugin) {
			allPluginsMap[plugin.name] = plugin;
		}
	};

	// If placeholders are allowed in the base configuration, gather the plugins.
	if (placeholdersInBase) {
		basePlugins.forEach(gatherPlugins);
	}

	// If placeholders are allowed in the override configuration, gather the plugins.
	if (placeholdersInOverride) {
		overridePlugins.forEach(gatherPlugins);
	}

	// Merge plugins
	const mergedPlugins: InputPluginOption = [];
	[...basePlugins, ...overridePlugins].forEach((plugin) => {
		// Replace placeholders with their respective plugin instances.
		if (typeof plugin === 'string') {
			const respectivePlugin = allPluginsMap[plugin];
			if (respectivePlugin != null) {
				mergedPlugins.push(respectivePlugin);
			} else {
				command.warn(`Plugin placeholder "${plugin}" does not match any available plugins.`);
			}
		}
		// Only add plugins that haven't been added yet.
		else if (
			(!isPlugin(plugin) || !mergedPlugins.some((p) => isPlugin(p) && p.name === plugin.name)) &&
			typeof plugin !== 'string' &&
			!(plugin instanceof Promise) // TODO: figure out how to handle Promise
		) {
			mergedPlugins.push(plugin);
		}
	});

	return mergedPlugins;
}

interface TMergeRollupConfigsConfig {
	command: Command;
	placeholdersInBase?: boolean;
	placeholdersInOverride?: boolean;
}
