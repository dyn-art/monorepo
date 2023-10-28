'use client';

import React from 'react';
import { createSVGComposition, initWasm, type Entity } from '@dyn/dtom';
import { MaxWidthWrapper } from '@/components';

interface RectangleState {
	x: number;
	y: number;
	dx: number;
	dy: number;
}

async function spawnAndMoveRectangles(rectangleCount: number): Promise<void> {
	await initWasm();

	const compositionHeight = 1000;
	const compositionWidth = 1000;
	const rectangleWidth = 100;
	const rectangleHeight = 100;

	const composition = createSVGComposition({ width: compositionWidth, height: compositionHeight });
	const rectangleStates: Record<Entity, RectangleState> = {};

	// Spawn rectangles at random positions
	for (let i = 0; i < rectangleCount; i++) {
		const x = Math.random() * (compositionWidth - rectangleWidth);
		const y = Math.random() * (compositionHeight - rectangleHeight);

		const rectangleEntity = composition.createRectangle({
			node: {
				node_type: 'Rectangle'
			},
			recangle: null,
			rectangle_corner_mixin: {
				top_left_radius: 5,
				top_right_radius: 5,
				bottom_right_radius: 5,
				bottom_left_radius: 5
			},
			composition_mixin: {
				is_visible: true,
				is_locked: false
			},
			layout_mixin: {
				width: rectangleWidth,
				height: rectangleHeight,
				relative_transform: [1, 0, x, 0, 1, y, 0, 0, 1]
			},
			blend_mixin: {
				blend_mode: 'Normal',
				opacity: 1,
				is_mask: false
			}
		});

		rectangleStates[rectangleEntity] = {
			x,
			y,
			dx: Math.random() > 0.5 ? 5 : -5,
			dy: Math.random() > 0.5 ? 5 : -5
		};
	}

	function animate() {
		for (const [rectangleEntity, state] of Object.entries(rectangleStates)) {
			// Update positions
			state.x += state.dx;
			state.y += state.dy;

			// Bounce off the wall
			if (state.x <= 0 || state.x >= compositionWidth - rectangleWidth) state.dx = -state.dx;
			if (state.y <= 0 || state.y >= compositionHeight - rectangleHeight) state.dy = -state.dy;

			// Move the entity
			composition.moveEntity(Number(rectangleEntity), state.dx, state.dy);
		}

		// Update the composition and schedule the next frame
		composition.update();
		requestAnimationFrame(animate);
	}

	// Start the animation
	animate();
}

async function onClick(): Promise<void> {
	await initWasm();
	await spawnAndMoveRectangles(10);
}

const Home: React.FC = () => {
	const [isLoading, setIsLoading] = React.useState(false);

	return (
		<MaxWidthWrapper>
			Our WASM component:
			{isLoading ? <div>Loading...</div> : null}
			<button
				onClick={() => {
					setIsLoading(true);
					onClick()
						.catch((err) => {
							console.log(err);
						})
						.finally(() => {
							setIsLoading(false);
						});
				}}
			>
				Load WASM
			</button>
		</MaxWidthWrapper>
	);
};

export default Home;
