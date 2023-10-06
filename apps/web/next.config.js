module.exports = {
	reactStrictMode: true,
	transpilePackages: ['ui'],
	webpack: (config, { isServer }) => {
		if (!isServer) {
			// Fix "Module not found: Can't resolve 'fs'" in Rust WASM bundle
			config.resolve.fallback = {
				...config.resolve.fallback,
				fs: false,
				path: false
			};
		}

		return config;
	}
};
