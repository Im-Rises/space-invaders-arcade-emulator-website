module.exports = {
	env: {
		browser: true,
		es2021: true,
	},
	extends: [
		'plugin:react/recommended',
		'xo',
	],
	overrides: [],
	parserOptions: {
		ecmaVersion: 'latest',
		sourceType: 'module',
	},
	plugins: [
		'react',
	],
	rules: {
		'eol-last': [
			'error',
			'always',
		],
		'function-call-argument-newline': 'off',
		'capitalized-comments': 'off',
		'react/prop-types': 'warn',
		'no-trailing-spaces': 0,
		'no-mixed-spaces-and-tabs': 'off',
		'max-len': ['error', {code: 140, ignoreComments: true}],
		'max-lines-per-function': ['error', {max: 140, skipBlankLines: true, skipComments: true}],
		'no-negated-condition': 'off',
		'max-params': [
			'error',
			6,
		],
	},
};
