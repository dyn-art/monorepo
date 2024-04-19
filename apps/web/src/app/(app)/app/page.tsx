'use client';

import React from 'react';
import type { COMP } from '@dyn/dtif-comp';
import { Editor } from '@/components';

const Page: React.FC = () => {
	return <Editor dtif={DEFAULT_DTIF} />;
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
			children: ['n2', 'n3', 'n4']
		},
		n2: {
			type: 'Rectangle',
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
					opacity: 0.5,
					paintId: 'p2'
				},
				{
					type: 'Fill',
					blendMode: 'Normal',
					opacity: 1,
					paintId: 'p5'
				},
				{
					type: 'Stroke',
					width: 20,
					blendMode: 'Normal',
					opacity: 1,
					paintId: 'p2'
				}
			],
			size: [100, 100],
			translation: [(WIDTH - 100) / 2, (HEIGHT - 100) / 2]
		},
		n3: {
			type: 'Rectangle',
			styles: [
				{
					type: 'Fill',
					blendMode: 'Normal',
					opacity: 1,
					paintId: 'p3'
				},
				{
					type: 'DropShadow',
					position: [4, 4],
					blur: 3,
					opacity: 0.3
				}
			],
			size: [100, 100],
			translation: [WIDTH / 2 - 200, HEIGHT / 2 + 100],
			cornerRadii: [0, 10, 20, 30]
		},
		n4: {
			type: 'Rectangle',
			styles: [
				{
					type: 'Fill',
					blendMode: 'Normal',
					opacity: 1,
					paintId: 'p3'
				}
			],
			size: [100, 100],
			translation: [0, 0],
			rotationDeg: 45
		}
	},
	paints: {
		p1: {
			type: 'Solid',
			color: [229, 229, 229]
		},
		p2: {
			type: 'Solid',
			color: [0, 128, 0]
		},
		p3: {
			type: 'Solid',
			color: [250, 128, 114]
		},
		p4: {
			type: 'Gradient',
			variant: { type: 'Linear' },
			stops: [
				{
					color: [138, 43, 226],
					position: 0
				},
				{
					color: [0, 191, 225],
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
				url: 'https://avatars.githubusercontent.com/u/108551717?s=200&v=4'
			},
			contentType: { type: 'Png' }
		}
	},
	events: [
		// {
		// 	type: 'EntityMoved',
		// 	entity: 'n1',
		// 	dx: 100,
		// 	dy: -300
		// }
	]
};
