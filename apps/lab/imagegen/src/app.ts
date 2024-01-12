import * as resvg from '@resvg/resvg-js';
import { Hono } from 'hono';
import satori from 'satori';

import { fetchFont } from './utils';

export const app = new Hono();

const inter = await fetchFont(
	'https://fonts.gstatic.com/s/inter/v13/UcCO3FwrK3iLTeHuS_fvQtMwCp50KnMw2boKoduKmMEVuLyfMZhrib2Bg-4.ttf'
);

app.post('/v1/satori', async (ctx) => {
	const format = ctx.req.query('format');
	const body = await ctx.req.json();

	// Loaad fonts

	// Generate SVG using Satori
	const svg = await satori(body, {
		width: 600,
		height: 400,
		fonts: [
			{
				name: 'Inter',
				data: inter,
				weight: 400,
				style: 'normal'
			}
		]
	});

	// Convert SVG to PNG
	if (format === 'png') {
		const renderer = new resvg.Resvg(svg, {
			fitTo: {
				mode: 'width',
				value: 600
			}
		});
		const pngData = renderer.render();
		const pngBuffer = pngData.asPng();

		return ctx.body(pngBuffer.buffer as ArrayBuffer, 200, {
			'Content-Type': 'image/png'
		});
	}
	// Default to sending SVG
	else {
		return ctx.body(svg, 200, {
			'Content-Type': 'image/svg+xml'
		});
	}
});
