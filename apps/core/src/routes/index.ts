import { Resvg } from '@resvg/resvg-js';
import { Router } from 'express';
import satori from 'satori';
import { z } from 'zod';
import { createOpenApiRouter } from '@dyn/openapi-router';
import type { paths } from '@dyn/types/core';

export const router: Router = Router();

const openApiRouter = createOpenApiRouter<paths>(router);

openApiRouter.get('/v1/ping', {}, async (req, res) => {
	res.status(200).send(true);
});

// eslint-disable-next-line @typescript-eslint/no-misused-promises -- Express
router.get('/v1/satori', async (req, res) => {
	const format = req.query.format;

	// Loaad fonts
	const fontResponse = await fetch(
		'https://fonts.gstatic.com/s/inter/v13/UcCO3FwrK3iLTeHuS_fvQtMwCp50KnMw2boKoduKmMEVuLyfMZhrib2Bg-4.ttf'
	);
	const fontBuffer = await fontResponse.arrayBuffer();

	// Generate SVG using Satori
	const svg = await satori(req.body, {
		width: 600,
		height: 400,
		fonts: [
			{
				name: 'Inter',
				data: fontBuffer,
				weight: 400,
				style: 'normal'
			}
		]
	});

	// Convert SVG to PNG
	if (format === 'png') {
		const resvg = new Resvg(svg, {
			fitTo: {
				mode: 'width',
				value: 600
			}
		});
		const pngData = resvg.render();
		const pngBuffer = pngData.asPng();

		res.setHeader('Content-Type', 'image/png');
		res.status(200).send(pngBuffer);
	}
	// Default to sending SVG
	else {
		res.setHeader('Content-Type', 'image/svg+xml');
		res.status(200).send(svg);
	}
});

openApiRouter.get(
	'/v1/hello',
	{
		querySchema: {
			name: z.string()
		}
	},
	async (req, res) => {
		const { name } = req.query;

		res.status(200).send(`Hello ${name}`);
	}
);
