/**
 * @type {import('eslint').Linter.Config}
 */
module.exports = {
	root: true,
	extends: [require.resolve('@ibg/config/eslint/react-internal'), 'plugin:storybook/recommended']
};
