import path from 'node:path';
import html from '@rollup/plugin-html';
import postcss from 'rollup-plugin-postcss';

import type { TBaseDynRollupOptions, TDynRollupOptionsCallbackConfig } from '../../types';

export async function createAppRollupConfig(
	config: TDynRollupOptionsCallbackConfig
): Promise<TBaseDynRollupOptions> {
	const { path: _path, output, packageJson, isProduction } = config;
	const bundleName = path.basename(_path.output).replace('.js', '');

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
					return `
<!doctype html>
<html lang="en" data-theme="lofi">
    <head>
        <meta charset="utf-8">
        <title>${packageJson.name ?? 'Figma Plugin'}</title>
    </head>
    <body>
        <script>
                                    ${
																			appBundle != null && 'code' in appBundle
																				? appBundle.code
																				: 'console.error("Failed to include app bundle!");'
																		}
        </script>
        <div id="root"></div>
    </body>
</html>
                    `;
				}
			}),
			// Process and bundle CSS files
			postcss({
				config: {
					path: './postcss.config.js',
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
