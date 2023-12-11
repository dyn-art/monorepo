/**
 * Merge custom Tailwind configuration with base configuration for `@dyn/ui`.
 * @param {object} tailwindConfig - Custom Tailwind config object
 * @return {object} Merged config object
 */
function withDynUI(tailwindConfig) {
	const baseConfig = require('./tailwind.config.js');

	return {
		...tailwindConfig,
		content: [
			...(tailwindConfig?.content ?? []),
			'./node_modules/@dyn/ui/dist/esm/components/**/*.{js,ts,jsx,tsx}'
		],
		presets: [...(tailwindConfig?.presets ?? []), baseConfig]
	};
}

module.exports = withDynUI;
