'use client';

import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import React from 'react';
import type { Composition } from '@dyn/svg-comp';

import { Canvas, type TCanvasProps } from './components';

const queryClient = new QueryClient();

export const Editor: React.FC<TEditorProps> = (props) => {
	const { width, height, dtif } = props;
	const [composition, setComposition] = React.useState<Composition | null>(null);

	return (
		<QueryClientProvider client={queryClient}>
			<div className="flex flex-col items-center justify-center">
				<Canvas dtif={dtif} height={height} onLoadedComposition={setComposition} width={width} />
			</div>
		</QueryClientProvider>
	);
};

export type TEditorProps = {
	// TODO:
} & Omit<TCanvasProps, 'onLoadedComposition'>;
