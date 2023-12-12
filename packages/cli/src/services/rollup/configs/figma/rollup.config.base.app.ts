import path from 'node:path';
import html from '@rollup/plugin-html';
import postcss from 'rollup-plugin-postcss';

import { readHtmlFile } from '../../../../utils';
import type { TBaseDynRollupOptions, TDynRollupOptionsCallbackConfig } from '../../../dyn';

export async function createAppRollupConfig(
	config: TDynRollupOptionsCallbackConfig & { postcssPath: string; rootHtmlPath?: string }
): Promise<TBaseDynRollupOptions> {
	const { path: _path, output, packageJson, isProduction, postcssPath, rootHtmlPath } = config;
	const bundleName = path.basename(_path.output).replace('.js', '');

	// Resolve Html to inject bundle into
	let rootHtml = `
	<!doctype html>
	<html lang="en">
		<head>
			<meta charset="utf-8">
			<title>${packageJson.name ?? 'Figma Plugin'}</title>
		</head>
		<body>
			<script>/* bundleCode */</script>
			<div id="root"></div>
		</body>
	</html>
`;
	if (rootHtmlPath != null) {
		const absoluteRootHtmlPath = path.resolve(process.cwd(), rootHtmlPath);
		const maybeRootHtml = await readHtmlFile(absoluteRootHtmlPath);
		if (maybeRootHtml != null) {
			rootHtml = maybeRootHtml;
		}
	}

	return {
		input: _path.input,
		output,
		plugins: [
			'node-resolve',
			'commonjs',
			'resolve-typescript-paths',
			'esbuild',
			// Generate HTML file with injected bundle
			html({
				fileName: `${bundleName}.html`,
				template(htmlTemplateOptions) {
					const appBundle = htmlTemplateOptions?.bundle[`${bundleName}.js`];
					return rootHtml.replace(
						'/* bundleCode */',
						appBundle != null && 'code' in appBundle
							? appBundle.code
							: '/* Failed to include app bundle! */'
					);
				}
			}),
			// Process and bundle CSS files
			postcss({
				config: {
					path: postcssPath,
					ctx: {}
				},
				minimize: isProduction,
				sourceMap: !isProduction
			}),
			'replace',
			'rollup-plugin-bundle-size'
		],
		external: []
	};
}
