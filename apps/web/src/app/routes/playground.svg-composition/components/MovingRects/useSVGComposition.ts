import React from 'react';
import { Composition, createSVGComposition, Entity, initWasm } from '@dyn/svg-composition';

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
				composition.clear();
			}
		};
	}, [width, height, count, svgContainerRef.current]);

	return { svgContainerRef, composition };
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

	// Set up your rectangles
	const rects: Record<
		string,
		{ x: number; y: number; size: number; speed: number; entity: Entity }
	> = {};
	for (let i = 0; i < count; i++) {
		const x = Math.random() * composition.width;
		const y = Math.random() * composition.height;
		const size = 10 + Math.random() * 40;
		const speed = 1 + Math.random();

		rects[i] = {
			x,
			y,
			size,
			speed,
			entity: composition.createRectangle({ x, y, width: size, height: size })
		};
	}

	// Animation loop
	const animate = () => {
		const rectKeysToRemove: string[] = [];

		for (const key in rects) {
			const rect = rects[key];
			if (rect == null) {
				continue;
			}

			rect.x -= rect.speed;
			composition.setEntityPosition(rect.entity, rect.x, rect.y);

			if (rect.x + rect.size / 2 < 0) {
				rectKeysToRemove.push(key);
			}
		}

		rectKeysToRemove.forEach((key) => {
			const rect = rects[key];
			if (rect != null) {
				rect.x = composition.width + rect.size / 2;
			}
		});

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
