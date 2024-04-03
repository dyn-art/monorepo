import React from 'react';

export function useSize<T extends HTMLElement = HTMLElement>(): [React.RefObject<T>, TSize | null] {
	const ref = React.useRef<T>(null);
	const [size, setSize] = React.useState<TSize | null>(null);

	React.useLayoutEffect(() => {
		const target = ref.current;
		if (target == null) {
			return;
		}

		const updateSize = (): void => {
			setSize({
				width: target.offsetWidth,
				height: target.offsetHeight
			});
		};

		updateSize();

		if (typeof ResizeObserver === 'undefined') {
			console.warn(
				"The 'useDivSize()' hook requires 'ResizeObserver'. Your browser does not support 'ResizeObserver'."
			);
			return;
		}

		const resizeObserver = new ResizeObserver(() => {
			updateSize();
		});
		resizeObserver.observe(target);

		return () => {
			resizeObserver.disconnect();
		};
	}, []);

	return [ref, size];
}

interface TSize {
	width: number;
	height: number;
}
