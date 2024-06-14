const withDynUi = require('@dyn/ui/with-tailwind');
const withDynCnvSvgEditor = require('@dyn/cnv-svg-editor/with-tailwind');

module.exports = withDynCnvSvgEditor(
	withDynUi(
		/** @type {import('tailwindcss').Config} */
		{
			content: ['./src/components/**/*.{ts,tsx}', './src/app/**/*.{ts,tsx}']
		}
	)
);
