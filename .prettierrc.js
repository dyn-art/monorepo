// @ts-check

/**
 * @type {import("@ianvs/prettier-plugin-sort-imports").PrettierConfig}
 */
module.exports = {
	// Standard prettier options
	useTabs: true,
	printWidth: 100,
	singleQuote: true,
	trailingComma: 'none',
	bracketSameLine: false,
	semi: true,
	quoteProps: 'consistent',
	plugins: ['@ianvs/prettier-plugin-sort-imports'],

	// prettier-plugin-sort-imports options
	// https://github.com/IanVS/prettier-plugin-sort-imports
	importOrder: [
		// External packages
		'<THIRD_PARTY_MODULES>',
		// Dyn packages
		'^@dyn/',
		// Internal packages
		'^@/',
		'^~/',
		'',
		// Relative
		'^[../]',
		'^[./]'
	],
	importOrderParserPlugins: ['typescript', 'jsx', 'decorators-legacy'],
	importOrderTypeScriptVersion: '5.2.2'
};
