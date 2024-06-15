// https://tailwindcss.com/docs/using-with-preprocessors
module.exports = {
	plugins: {
		'tailwindcss': {
			config: './tailwind.config.js'
		},
		'autoprefixer': {},
		'postcss-nested': {}
	}
};
