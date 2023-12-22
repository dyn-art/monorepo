import { Composition, createSVGComposition, initWasm } from '@dyn/svg-composition';

import { ABEEZEE_ITALIC, INTER_REGULAR, loadFonts } from '../../font';
import { TEST_DATA } from './test-data';

export async function createComposition(config: {
	width: number;
	height: number;
	element: Element;
}): Promise<Composition> {
	const { width, height, element } = config;
	const fonts = await loadFonts([INTER_REGULAR, ABEEZEE_ITALIC]);

	await initWasm();

	const composition = createSVGComposition({
		width,
		height,
		renderer: {
			domElement: element
		},
		dtif: TEST_DATA(width, height, fonts)
	});

	return composition;
}
