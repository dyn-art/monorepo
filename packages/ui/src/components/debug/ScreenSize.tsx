'use client';

// https://gist.github.com/Sh4yy/0300299ae60af4910bcb341703946330
import React from 'react';

export const ScreenSize: React.FC = () => {
	const [dimensions, setDimensions] = React.useState({ width: 0, height: 0 });

	React.useEffect(() => {
		function updateDimensions(): void {
			setDimensions({
				width: window.innerWidth,
				height: window.innerHeight
			});
		}

		updateDimensions();
		window.addEventListener('resize', updateDimensions);

		return () => {
			window.removeEventListener('resize', updateDimensions);
		};
	}, []);

	const { width, height } = dimensions;

	return (
		<div className="fixed bottom-5 left-5 z-50 flex items-center space-x-2 rounded-full bg-black px-2.5 py-1 font-mono text-xs font-medium text-white">
			<span>
				{width.toLocaleString()} x {height.toLocaleString()}
			</span>
			<div className="h-4 w-px bg-gray-800" />
			<span className="sm:hidden">XS</span>
			<span className="hidden sm:inline md:hidden">SM</span>
			<span className="hidden md:inline lg:hidden">MD</span>
			<span className="hidden lg:inline xl:hidden">LG</span>
			<span className="hidden xl:inline 2xl:hidden">XL</span>
			<span className="hidden 2xl:inline">2XL</span>
		</div>
	);
};
