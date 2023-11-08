import React from 'react';
import { Composition, createSVGComposition, Entity, initWasm } from '@dyn/dtom';

export const useSVGComposition = (props: UseSVGCompositionProps) => {
	const { width, height, count = 50 } = props;
	const svgContainerRef = React.useRef<HTMLDivElement>(null);
	const [composition, setComposition] = React.useState<Composition | null>(null);

	let isMounted = true; // https://github.com/facebook/react/issues/24502
	React.useEffect(() => {
		(async () => {
			if (svgContainerRef.current && composition == null && isMounted) {
				const newComposition = await createComposition({
					width,
					height,
					element: svgContainerRef.current
				});
				setComposition(newComposition);
				startLoop({ composition: newComposition, count });
			}
		})();
		return () => {
			isMounted = false;
			if (composition != null) {
				composition.destory();
			}
		};
	}, [width, height, count, svgContainerRef.current]);

	return svgContainerRef;
};

async function createComposition(config: {
	width: number;
	height: number;
	element: Element;
}): Promise<Composition> {
	const { width, height, element } = config;
	await initWasm();

	const composition = createSVGComposition({
		width,
		height,
		renderer: {
			domElement: element
		}
	});

	return composition;
}

function startLoop(config: { count: number; composition: Composition }) {
	const { count, composition } = config;
	const rectWidth = 50;
	const rectHeight = 50;

	// Set up your rectangles
	const rects: Record<Entity, { x: number; y: number; dx: number; dy: number }> = {};
	for (let i = 0; i < count; i++) {
		const x = Math.random() * (composition.width - rectWidth);
		const y = Math.random() * (composition.height - rectHeight);
		const entity = composition.createRectangle({ x, y, width: rectWidth, height: rectHeight });

		rects[entity] = {
			x,
			y,
			dx: Math.random() > 0.5 ? 5 : -5,
			dy: Math.random() > 0.5 ? 5 : -5
		};
	}

	// Spawn non moving rects
	for (let i = 0; i < count; i++) {
		const x = Math.random() * (composition.width - rectWidth);
		const y = Math.random() * (composition.height - rectHeight);
		composition.createRectangle({ x, y, width: rectWidth, height: rectHeight });
	}

	// Animation loop
	const animate = () => {
		for (const [rectangleEntity, state] of Object.entries(rects)) {
			// Update positions
			state.x += state.dx;
			state.y += state.dy;

			// Bounce off the walls, account for rectangle size
			if (state.x <= 0 || state.x + rectWidth >= composition.width) state.dx = -state.dx;
			if (state.y <= 0 || state.y + rectHeight >= composition.height) state.dy = -state.dy;

			// Move the entity to its updated position
			composition.setEntityPosition(Number(rectangleEntity), state.x, state.y);
		}

		composition.update();
		requestAnimationFrame(animate);
	};

	// Start animation loop
	requestAnimationFrame(animate);
}

type UseSVGCompositionProps = {
	width: number;
	height: number;
	count?: number;
};
