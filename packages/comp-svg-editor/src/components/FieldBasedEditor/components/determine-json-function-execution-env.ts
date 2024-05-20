import type { TJsonFunction } from '@dyn/utils';

export function deterimeJsonFunctionExecutionEnv(jsonFunction: TJsonFunction): 'iframe' | 'direct' {
	const { args, body } = jsonFunction;

	// Extract all identifiers from the body
	const identifierRegex = /\b[a-zA-Z_$][a-zA-Z0-9_$]*\b/g;
	const identifiers = body.match(identifierRegex) || [];

	// JavaScript keywords and built-ins whitelist
	const identifiersWhitelist = new Set([
		'const',
		'let',
		'return',
		'if',
		'else',
		'for',
		'while',
		'do',
		'switch',
		'case',
		'default',
		'break',
		'continue',
		'throw',
		'try',
		'catch',
		'finally',
		'new',
		'delete',
		'null',
		'true',
		'false',
		'undefined',
		...args
	]);

	for (const identifier of identifiers) {
		if (!identifiersWhitelist.has(identifier)) {
			return 'iframe';
		}
	}

	return 'direct';
}
