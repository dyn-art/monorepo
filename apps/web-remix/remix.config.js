/** @type {import('@remix-run/dev').AppConfig} */
export default {
	ignoredRouteFiles: ['**/.*', '**/*.css', '**/*.scss'],
	appDirectory: './src/app',
	assetsBuildDirectory: 'public/build',
	// publicPath: "/build/",
	// serverBuildPath: "build/index.js",
	tailwind: true,
	postcss: true
};