const OFF = 0;
const WARNING = 1;
const ERROR = 2;

const { resolve } = require('node:path');

const project = resolve(process.cwd(), 'tsconfig.json');

/**
 * @type {import('eslint').Linter.Config}
 */
module.exports = {
	extends: [require.resolve('@vercel/style-guide/eslint/typescript'), 'turbo'],
	parserOptions: {
		project
	},
	settings: {
		'import/resolver': {
			typescript: {
				project
			}
		}
	},
	rules: {
		// Typescript
		'@typescript-eslint/naming-convention': WARNING,
		'@typescript-eslint/no-unused-vars': WARNING,
		'@typescript-eslint/ban-ts-comment': WARNING,
		'@typescript-eslint/require-await': WARNING,
		'@typescript-eslint/no-unsafe-member-access': WARNING,
		'@typescript-eslint/no-unsafe-return': WARNING,
		'@typescript-eslint/unbound-method': WARNING,
		'@typescript-eslint/no-unsafe-call': WARNING,

		// EsLint
		'no-console': WARNING,
		'import/no-default-export': WARNING,
		'eqeqeq': OFF,
		'import/order': OFF, // Handled by prettier
		'import/no-extraneous-dependencies': WARNING
	}
};
