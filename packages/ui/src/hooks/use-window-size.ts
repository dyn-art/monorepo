import React from 'react';

export function useWindowSize(): TSize | null {
	const [windowSize, setWindowSize] = React.useState<TSize | null>(
		typeof window !== 'undefined'
			? {
					width: window.innerWidth,
					height: window.innerHeight
				}
			: null
	);

	React.useLayoutEffect(() => {
		if (typeof window === 'undefined') {
			console.warn("The 'useWindowSize()' hook requires the 'window' global object!");
			return;
		}

		const handleResize = (): void => {
			setWindowSize({
				width: window.innerWidth,
				height: window.innerHeight
			});
		};

		handleResize();

		window.addEventListener('resize', handleResize);

		return () => {
			window.removeEventListener('resize', handleResize);
		};
	}, []);

	return windowSize;
}

interface TSize {
	width: number;
	height: number;
}
