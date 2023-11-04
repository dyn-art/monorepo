// Based on: https://github.com/smplrspace/react-fps-stats

import React, { useEffect, useRef } from 'react';

import { useFPSStatsReducer } from './useFPSStatsReducer';

const FPSStats: React.FC<TProps> = ({ graphHeight = 29, graphWidth = 70 }) => {
	const [state, dispatch] = useFPSStatsReducer(graphWidth);

	const requestRef = useRef<number>();

	const tick = () => {
		dispatch();
		requestRef.current = requestAnimationFrame(tick);
	};

	useEffect(() => {
		requestRef.current = requestAnimationFrame(tick);
		return () => {
			if (requestRef.current) {
				cancelAnimationFrame(requestRef.current);
			}
		};
	}, []);

	const { fps, max, len } = state;

	return (
		<div
			className={`pointer-events-none relative h-12 overflow-hidden bg-black p-1 text-xs font-bold leading-none text-yellow-300`}
			style={{
				width: graphWidth + 6
			}}
		>
			<span>{fps[len - 1]} FPS</span>
			<div className={`box-border h-${graphHeight} bg-gray-800`}>
				{fps.map((frame, i) => (
					<div
						key={`fps-${i}`}
						className={`absolute bottom-0 box-border bg-yellow-300`}
						style={{
							right: `${len - 1 - i}px`,
							height: `${(graphHeight * frame) / max}px`,
							width: 1
						}}
					/>
				))}
			</div>
		</div>
	);
};

export { FPSStats };

interface TProps {
	top?: number | string;
	right?: number | string;
	bottom?: number | string;
	left?: number | string;
	graphHeight?: number;
	graphWidth?: number;
}
