'use client';

import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import React from 'react';

import { InnerEditor, type TInnerEditorProps } from './components';

const queryClient = new QueryClient();

export const Editor: React.FC<TEditorProps> = (props) => {
	return (
		<QueryClientProvider client={queryClient}>
			<InnerEditor {...props} />
		</QueryClientProvider>
	);
};

export type TEditorProps = TInnerEditorProps;
