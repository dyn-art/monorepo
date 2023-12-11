const withDynUI = require('@dyn/ui/with-dyn-ui');

module.exports = withDynUI(
	/** @type {import('tailwindcss').Config} */
	{
		content: ['./src/app/**/*.{ts,tsx}']
	}
);
