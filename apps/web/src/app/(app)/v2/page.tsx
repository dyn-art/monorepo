import React from 'react';
import type { COMP } from '@dyn/comp-dtif';
import { Editor } from '@/components';

const Page: React.FC = () => {
	return (
		<div>
			<Editor dtif={DEFAULT_DTIF} height={HEIGHT} width={WIDTH} />
		</div>
	);
};

export default Page;

const WIDTH = 1500;
const HEIGHT = 500;
const DEFAULT_DTIF: COMP.DtifComposition = {
	size: [WIDTH, HEIGHT],
	rootNodeId: 'n1',
	nodes: {
		n1: {
			type: 'Frame',
			styles: [
				{
					type: 'Fill',
					blendMode: 'Normal',
					opacity: 1.0,
					paintId: 'p1'
				}
			],
			size: [WIDTH, HEIGHT],
			translation: [0, 0],
			children: ['n2']
		},
		n2: {
			type: 'Rectangle',
			cornerRadii: [10, 20, 30, 40],
			styles: [
				// {
				// 	type: 'Stroke',
				// 	width: 5,
				// 	blendMode: 'Normal',
				// 	opacity: 1,
				// 	paintId: 'p3'
				// },
				{
					type: 'Fill',
					blendMode: 'Normal',
					opacity: 0.9,
					paintId: 'p2'
				}
				// {
				// 	type: 'Fill',
				// 	blendMode: 'Normal',
				// 	opacity: 0.8,
				// 	paintId: 'p5'
				// },
				// {
				// 	type: 'Stroke',
				// 	width: 20,
				// 	blendMode: 'Normal',
				// 	opacity: 0.7,
				// 	paintId: 'p2'
				// }
			],
			size: [100, 100],
			translation: [(WIDTH - 100) / 2, (HEIGHT - 100) / 2],
			angle: 45
		}
	},
	paints: {
		p1: {
			type: 'Solid',
			color: {
				red: 229,
				green: 229,
				blue: 229
			}
		},
		p2: {
			type: 'Solid',
			color: {
				red: 0,
				green: 128,
				blue: 0
			}
		},
		p3: {
			type: 'Solid',
			color: {
				red: 250,
				green: 128,
				blue: 114
			}
		},
		p4: {
			type: 'Gradient',
			variant: { type: 'Linear' },
			stops: [
				{
					color: {
						red: 138,
						green: 43,
						blue: 226
					},
					position: 0
				},
				{
					color: {
						red: 0,
						green: 191,
						blue: 225
					},
					position: 1
				}
			]
		},
		p5: {
			type: 'Image',
			assetId: 'a1'
		}
	},
	assets: {
		a1: {
			content: {
				type: 'Url',
				url: 'https://raw.githubusercontent.com/dyndotart/monorepo/develop/docs/resources/images/logo-rounded.png'
			},
			contentType: { type: 'Png' }
		}
	},
	events: [
		// {
		// 	type: 'EntityMoved',
		// 	entity: 1,
		// 	dx: 100,
		// 	dy: -300
		// }
	]
};
