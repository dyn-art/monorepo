const withDynUi = require('@dyn/ui/with-dyn-ui');

module.exports = withDynUi(
	/** @type {import('tailwindcss').Config} */
	{
		content: ['./src/app/**/*.{ts,tsx}']
	}
);
