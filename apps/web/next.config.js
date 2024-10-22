const withBundleAnalyzer = require('@next/bundle-analyzer')({
	enabled: process.env.ANALYZE === 'true'
});

/** @type {import('next').NextConfig} */
const nextConfig = {
	pageExtensions: ['js', 'jsx', 'ts', 'tsx'],
	reactStrictMode: true,
	webpack: (config) => {
		// https://github.com/kelektiv/node.bcrypt.js/issues/979
		config.externals = [...config.externals, 'bcrypt'];
		return config;
	},
	rewrites: async () => {
		const rewrites = {
			afterFiles: [
				// Apply any of your existing rewrites here
			],
			fallback: []
		};

		// DEV only, this allows for local api calls to be proxied to
		// api routes that use Rust runtime
		// https://javascript.plainenglish.io/integrating-rust-into-next-js-how-to-developer-guide-10e533470d71
		if (process.env.NODE_ENV === 'development') {
			rewrites.fallback.push({
				source: '/api/graphic/:path*',
				destination: 'http://0.0.0.0:3001/api/graphic/:path*'
			});
		}

		return rewrites;
	}
};

module.exports = withBundleAnalyzer(nextConfig);
