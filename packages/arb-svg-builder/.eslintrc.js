/**
 * @type {import('eslint').Linter.Config}
 */
module.exports = {
	root: true,
	extends: [require.resolve('@ibg/config/eslint/library')],
	ignorePatterns: ['**/rust_modules/**/*']
};
