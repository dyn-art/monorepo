/**
 * @type {import('eslint').Linter.Config}
 */
module.exports = {
	root: true,
	extends: [require.resolve('@dyn/config/eslint/library')],
	ignorePatterns: ['**/rust_modules/**/*']
};
