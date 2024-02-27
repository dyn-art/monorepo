import React from 'react';
import type { COMP } from '@dyn/comp-dtif';
import { Editor } from '@/components';

const Page: React.FC = () => {
	return (
		<div>
			Hello World <Editor dtif={DEFAULT_DTIF} height={100} width={100} />{' '}
		</div>
	);
};

export default Page;

const WIDTH = 1500;
const HEIGHT = 500;
const DEFAULT_DTIF: COMP.CompDtif = {
	version: '0.0.1',
	name: 'Test',
	size: [WIDTH, HEIGHT],
	rootNodeId: 'n1',
	nodes: {
		n1: {
			type: 'Frame',
			angleInRadians: 0,
			blendMode: 'Normal',
			clipContent: false,
			cornerRadii: [0, 0, 0, 0],
			fill: [
				{
					blendMode: 'Normal',
					opacity: 1,
					paintId: 'p1'
				}
			],
			opacity: 1,
			size: [WIDTH, HEIGHT],
			stroke: [],
			translation: [0, 0],
			visibility: 'Visible',
			children: ['n2']
		},
		n2: {
			type: 'Rectangle',
			angleInRadians: 0,
			blendMode: 'Normal',
			cornerRadii: [0, 0, 0, 0],
			fill: [
				{
					blendMode: 'Normal',
					opacity: 1,
					paintId: 'p2'
				}
			],
			opacity: 1,
			size: [100, 100],
			stroke: [],
			translation: [WIDTH / 2, HEIGHT / 2],
			visibility: 'Visible'
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
