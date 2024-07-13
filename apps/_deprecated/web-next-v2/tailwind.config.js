const withDynUi = require('@dyn/ui/with-tailwind');
const withDynArbSvgEditor = require('@dyn/arb-svg-editor/with-tailwind');

module.exports = withDynArbSvgEditor(
	withDynUi(
		/** @type {import('tailwindcss').Config} */
		{
			content: ['./src/components/**/*.{ts,tsx}', './src/app/**/*.{ts,tsx}']
		}
	)
);
