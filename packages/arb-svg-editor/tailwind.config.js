const withDynUi = require('@dyn/ui/with-tailwind');

module.exports = withDynUi(
	/** @type {import('tailwindcss').Config} */
	{
		content: ['./src/components/**/*.{ts,tsx}']
	}
);
