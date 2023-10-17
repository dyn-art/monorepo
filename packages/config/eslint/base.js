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
		'@typescript-eslint/no-dynamic-delete': WARNING,
		'@typescript-eslint/ban-types': WARNING,
		'@typescript-eslint/no-explicit-any': WARNING,
		'@typescript-eslint/no-unsafe-assignment': WARNING,

		// EsLint
		'no-console': WARNING,
		'import/no-default-export': WARNING,
		'eqeqeq': OFF, // Often use it to check against null & undefined (e.g. 'var == null')
		'import/order': OFF, // Handled by prettier
		'import/no-extraneous-dependencies': WARNING,
		'no-await-in-loop': WARNING,
		'no-bitwise': WARNING,
		'unicorn/filename-case': OFF, // Annoying with React components and Typescript classes
		'import/no-named-as-default-member': OFF, // For ReactJs imports like React.useState()
		'import/no-extraneous-dependencies': WARNING
	}
};
