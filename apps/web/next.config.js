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
	}
};

module.exports = withBundleAnalyzer(nextConfig);
