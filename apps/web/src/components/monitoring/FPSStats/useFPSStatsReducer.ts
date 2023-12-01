import React from 'react';

export function useFPSStatsReducer(graphWidth: number) {
	const reducer = (state: TState): TState => {
		const currentTime = Date.now();
		if (currentTime > state.prevTime + 1000) {
			const nextFPS = [
				...new Array(Math.floor((currentTime - state.prevTime - 1000) / 1000)).fill(0),
				Math.max(1, Math.round((state.frames * 1000) / (currentTime - state.prevTime)))
			];
			return {
				max: Math.max(state.max, ...nextFPS),
				len: Math.min(state.len + nextFPS.length, graphWidth),
				fps: [...state.fps, ...nextFPS].slice(-graphWidth),
				frames: 1,
				prevTime: currentTime
			};
		} else {
			return { ...state, frames: state.frames + 1 };
		}
	};

	return React.useReducer(reducer, {
		len: 0,
		max: 0,
		frames: 0,
		prevTime: Date.now(),
		fps: []
	});
}

interface TState {
	len: number;
	max: number;
	frames: number;
	prevTime: number;
	fps: number[];
}
