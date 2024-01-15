'use client';

import dynamic from 'next/dynamic';
import React from 'react';
import { mat3, vec3, type Composition as TComposition } from '@dyn/svg-composition';

export function createTransformMatrix(x: number, y: number, angleDegrees: number) {
	const angleRadians = (angleDegrees * Math.PI) / 180; // Convert angle to radians

	return mat3(
		vec3(Math.cos(angleRadians), -Math.sin(angleRadians), 0),
		vec3(Math.sin(angleRadians), Math.cos(angleRadians), 0),
		vec3(x, y, 1)
	);
}

export const Composition = dynamic(
	async () => {
		const { initWasm, createSVGComposition } = await import('@dyn/svg-composition');

		await initWasm();

		// eslint-disable-next-line react/function-component-definition, react/display-name -- j
		return () => {
			const svgContainerRef = React.useRef<HTMLDivElement>(null);
			const [composition, setComposition] = React.useState<TComposition | null>(null);
			const width = 500;
			const height = 500;

			React.useEffect(() => {
				if (svgContainerRef.current != null && composition == null) {
					const newComposition = createSVGComposition({
						width,
						height,
						renderer: {
							domElement: svgContainerRef.current
						},
						dtif: {
							version: '0.0.1',
							name: 'Test',
							width,
							height,
							rootNodeId: 0,
							nodes: {
								0: {
									type: 'Frame',
									children: [1, 2, 3],
									dimension: {
										width,
										height
									},
									relativeTransform: createTransformMatrix(0, 0, 0)
								},
								1: {
									type: 'Rectangle',
									compositionMixin: { isVisible: true, isLocked: false },
									dimension: {
										width: 100,
										height: 100
									},
									relativeTransform: createTransformMatrix(
										(width - 100) / 2,
										(height - 100) / 2,
										30
									),
									rectangleCornerMixin: {
										bottomLeftRadius: 20,
										bottomRightRadius: 0,
										topLeftRadius: 0,
										topRightRadius: 0
									},
									fill: {
										paintIds: [5]
									}
								},
								2: {
									type: 'Rectangle',
									compositionMixin: { isVisible: true, isLocked: false },
									dimension: {
										width: 150,
										height: 150
									},
									relativeTransform: createTransformMatrix(
										(width - 100) / 1.5,
										(height - 100) / 1.5,
										0
									),
									rectangleCornerMixin: {
										bottomLeftRadius: 10,
										bottomRightRadius: 20,
										topLeftRadius: 40,
										topRightRadius: 80
									},
									fill: {
										paintIds: [5]
									}
								}
							},
							paints: {
								5: {
									type: 'Solid',
									blendMode: 'Normal',
									color: [189, 183, 107],
									isVisible: true,
									opacity: 1
								}
							},
							fonts: {},
							changes: [
								// {
								// 	type: 'EntityMoved',
								// 	entity: 1,
								// 	dx: 100,
								// 	dy: -300
								// }
							]
						}
					});
					setComposition(newComposition);
					newComposition.update();
				}

				return () => {
					if (composition != null) {
						composition.unmount();
					}
				};
			}, [composition]);

			return (
				<div>
					<p>Hello World</p>
					<div ref={svgContainerRef} />
				</div>
			);
		};
	},
	{
		loading: () => <p>Loading...</p>,
		ssr: false
	}
);
