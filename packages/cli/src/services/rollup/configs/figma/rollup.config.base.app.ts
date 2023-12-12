import path from 'node:path';
import html from '@rollup/plugin-html';
import postcss from 'rollup-plugin-postcss';
import type { PackageJson } from 'type-fest';

import { readHtmlFile } from '../../../../utils';
import type { TBaseDynRollupOptions, TDynRollupOptionsCallbackConfig } from '../../../dyn';

export async function createAppRollupConfig(
	config: TDynRollupOptionsCallbackConfig & { postcssPath: string; rootHtmlPath?: string }
): Promise<TBaseDynRollupOptions> {
	const { path: _path, output, packageJson, isProduction, postcssPath, rootHtmlPath } = config;
	const bundleName = path.basename(_path.output).replace('.js', '');

	// Construct or fetch the root HTML template
	const rootHtml = await getRootHtml(rootHtmlPath, packageJson);

	return {
		input: _path.input,
		output,
		plugins: [
			'node-resolve',
			'commonjs',
			'resolve-typescript-paths',
			'esbuild',
			// Inject the bundle into HTML template
			html({
				fileName: `${bundleName}.html`,
				template(htmlTemplateOptions) {
					const appBundle = htmlTemplateOptions?.bundle[`${bundleName}.js`];
					return rootHtml.replace(
						'/* bundleCode */',
						appBundle && 'code' in appBundle
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

async function getRootHtml(
	rootHtmlPath: string | undefined,
	packageJson: PackageJson
): Promise<string> {
	// Default HTML template
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

	// Load custom HTML template if provided
	if (rootHtmlPath) {
		const absoluteRootHtmlPath = path.resolve(process.cwd(), rootHtmlPath);
		const maybeRootHtml = await readHtmlFile(absoluteRootHtmlPath);
		if (maybeRootHtml) {
			rootHtml = maybeRootHtml;
		}
	}

	return rootHtml;
}
