import type { Plugin } from 'rollup';

export function isPlugin(obj: unknown): obj is Plugin {
	return typeof obj === 'object' && obj !== null && 'name' in obj;
}
