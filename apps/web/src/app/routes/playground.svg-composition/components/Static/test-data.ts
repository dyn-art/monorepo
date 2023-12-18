import { DTIFComposition, FontWithContent } from '@dyn/svg-composition';

import { ABEEZEE_ITALIC, INTER_REGULAR } from '../../font';
import { createTransformMatrix } from '../../utils';

export const COMPOSITION_WITH_ONE_RECT = (
	width: number,
	height: number,
	fonts: [number, FontWithContent][]
): DTIFComposition => ({
	version: '0.0.1',
	name: 'Test',
	width,
	height,
	rootNodeId: 0,
	nodes: [
		[
			0,
			{
				type: 'Frame',
				children: [1, 2, 3],
				dimension: {
					width,
					height
				},
				relativeTransform: createTransformMatrix(0, 0, 0),
				fill: {
					paintIds: [10]
				}
			}
		],
		[
			1,
			{
				type: 'Rectangle',
				compositionMixin: { isVisible: true, isLocked: false },
				dimension: {
					width: 100,
					height: 100
				},
				relativeTransform: createTransformMatrix((width - 100) / 2, (height - 100) / 2, 30),
				fill: {
					paintIds: [11]
				}
			}
		],
		[
			2,
			{
				type: 'Rectangle',
				compositionMixin: { isVisible: true, isLocked: false },
				dimension: {
					width: 100,
					height: 100
				},
				relativeTransform: createTransformMatrix((width - 100) / 2, (height - 100) / 2, 0),
				fill: {
					paintIds: [12]
				}
			}
		],
		[
			3,
			{
				type: 'Text',
				text: {
					sections: [
						{
							value: 'Hello there ',
							style: {
								fontId: INTER_REGULAR.id,
								fontSize: 48
							}
						},
						{
							value: 'Jeff',
							style: {
								fontId: ABEEZEE_ITALIC.id,
								fontSize: 70
							}
						},
						{
							value: '! Long line test testtest',
							style: {
								fontId: INTER_REGULAR.id,
								fontSize: 48
							}
						},
						{
							value: 'Extra small',
							style: {
								fontId: INTER_REGULAR.id,
								fontSize: 24
							}
						}
					]
				},
				compositionMixin: { isVisible: true, isLocked: false },
				dimension: {
					width: 500,
					height: 300
				},
				relativeTransform: createTransformMatrix((width - 100) / 4, (height - 100) / 4, 0),
				fill: {
					paintIds: [11]
				}
			}
		]
	],
	paints: [
		[
			10,
			{
				type: 'Solid',
				blendMode: 'Normal',
				color: [169, 169, 169],
				isVisible: true,
				opacity: 1
			}
		],
		[
			11,
			{
				type: 'Solid',
				blendMode: 'Normal',
				color: [255, 0, 0],
				isVisible: true,
				opacity: 1
			}
		],
		[
			12,
			{
				type: 'Solid',
				blendMode: 'Normal',
				color: [0, 0, 139],
				isVisible: true,
				opacity: 1
			}
		]
	],
	fonts,
	changes: [
		// {
		// 	type: 'EntityMoved',
		// 	entity: 1,
		// 	dx: 100,
		// 	dy: -300
		// }
	]
});
