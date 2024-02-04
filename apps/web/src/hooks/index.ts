'use client';

import React, { useEffect } from 'react';

export function useWindowSize(): TSize {
	// Initialize state with undefined width/height so server and client renders match
	// https://joshwcomeau.com/react/the-perils-of-rehydration/
	const [windowSize, setWindowSize] = React.useState<TSize>({
		width: typeof window !== 'undefined' ? window.innerWidth : null,
		height: typeof window !== 'undefined' ? window.innerHeight : null
	});

	useEffect(() => {
		function handleResize(): void {
			setWindowSize({
				width: window.innerWidth,
				height: window.innerHeight
			});
		}

		// Add event listener
		window.addEventListener('resize', handleResize);
		handleResize();

		// Remove event listener on cleanup
		return () => {
			window.removeEventListener('resize', handleResize);
		};
	}, []);

	return windowSize;
}

interface TSize {
	width: number | null;
	height: number | null;
}
