import React from 'react';
import Two from 'two.js';

export const useSVGComposition = (props: UseSVGCompositionProps) => {
	const { width, height, count = 50 } = props;
	const svgContainerRef = React.useRef<HTMLDivElement>(null);
	const [twoJs, setTwoJs] = React.useState<Two | null>(null);

	let isMounted = true; // https://github.com/facebook/react/issues/24502
	React.useEffect(() => {
		(async () => {
			if (svgContainerRef.current && twoJs == null && isMounted) {
				const newTwoJs = await createTwoJs({
					width,
					height,
					element: svgContainerRef.current
				});
				setTwoJs(newTwoJs);
				startLoop({ twoJs: newTwoJs, width, height, count });
			}
		})();
		return () => {
			isMounted = false;
			if (twoJs != null) {
				twoJs.unbind();
			}
		};
	}, [width, height, count, svgContainerRef.current]);

	return svgContainerRef;
};

async function createTwoJs(config: {
	width: number;
	height: number;
	element: HTMLElement;
}): Promise<Two> {
	const { width, height, element } = config;

	return new Two({
		width: width,
		height: height,
		type: Two.Types['svg'],
		autostart: true
	}).appendTo(element);
}

function startLoop(config: { count: number; width: number; height: number; twoJs: Two }) {
	const { count, width, height, twoJs } = config;

	// Set up your rectangles
	const rects: Record<string, { x: number; y: number; size: number; speed: number; el: any }> = {};
	for (let i = 0; i < count; i++) {
		const x = Math.random() * width;
		const y = Math.random() * height;
		const size = 10 + Math.random() * 40;
		const speed = 1 + Math.random();

		rects[i] = {
			x,
			y,
			size,
			speed,
			el: twoJs.makeRectangle(x, y, size, size)
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
			rect.el.translation.set(rect.x, rect.y);

			if (rect.x + rect.size / 2 < 0) {
				rectKeysToRemove.push(key);
			}
		}

		rectKeysToRemove.forEach((key) => {
			const rect = rects[key];
			if (rect != null) {
				rect.x = twoJs.width + rect.size / 2;
			}
		});

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
