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
const DEFAULT_DTIF: COMP.CompDtif = {
	size: [WIDTH, HEIGHT],
	rootNodeId: 'n1',
	nodes: {
		n1: {
			type: 'Frame',
			blendMode: 'Normal',
			clipContent: false,
			cornerRadii: [0, 0, 0, 0],
			styles: [
				{
					type: 'Fill',
					blendMode: 'Normal',
					opacity: 1.0,
					paintId: 'p1'
				}
			],
			opacity: 1,
			size: [WIDTH, HEIGHT],
			translation: [0, 0],
			visibility: 'Visible',
			children: ['n2']
		},
		n2: {
			type: 'Rectangle',
			blendMode: 'Normal',
			cornerRadii: [0, 20, 0, 0],
			styles: [
				{
					type: 'Stroke',
					width: 5,
					blendMode: 'Normal',
					opacity: 1,
					paintId: 'p3'
				},
				{
					type: 'Fill',
					blendMode: 'Normal',
					opacity: 0.9,
					paintId: 'p2'
				},
				{
					type: 'Fill',
					blendMode: 'Normal',
					opacity: 0.8,
					paintId: 'p1'
				},
				{
					type: 'Stroke',
					width: 20,
					blendMode: 'Normal',
					opacity: 0.7,
					paintId: 'p2'
				}
			],
			opacity: 1,
			size: [100, 100],
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
		},
		p3: {
			type: 'Solid',
			color: {
				red: 250,
				green: 128,
				blue: 114
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
