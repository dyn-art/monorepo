/**
 * Merge custom Tailwind configuration with base configuration for `@dyn/comp-svg-editor`.
 * @param {object} tailwindConfig - Custom Tailwind config object
 * @return {object} Merged config object
 */
function withTailwind(tailwindConfig) {
	const newTailwindConfig = {
		...tailwindConfig,
		content: [
			...(tailwindConfig?.content ?? []),
			'./node_modules/@dyn/comp-svg-editor/dist/esm/components/**/*.{js,ts,jsx,tsx}'
		]
	};

	if (!tailwindConfig.withDynUi) {
		const baseConfig = require('./tailwind.config.js');
		newTailwindConfig.presets = [...(tailwindConfig?.presets ?? []), baseConfig];
	}

	return newTailwindConfig;
}

module.exports = withTailwind;
