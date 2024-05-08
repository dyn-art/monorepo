const withDynUi = require('@dyn/ui/with-tailwind');
const withDynCompSvgEditor = require('@dyn/comp-svg-editor/with-tailwind');

module.exports = withDynCompSvgEditor(
	withDynUi(
		/** @type {import('tailwindcss').Config} */
		{
			content: ['./src/components/**/*.{ts,tsx}', './src/app/**/*.{ts,tsx}']
		}
	)
);
