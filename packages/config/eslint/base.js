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

		// EsLint
		'no-console': WARNING
	}
};
