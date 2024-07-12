'use client';

import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import React from 'react';
import { CanvaStyleEditor } from '@dyn/arb-svg-editor';
import { LayoutWrapper } from '@dyn/ui';

const queryClient = new QueryClient();

const Page: React.FC = () => {
	return (
		<LayoutWrapper size="full" className="h-screen" asChild>
			<main>
				<QueryClientProvider client={queryClient}>
					<CanvaStyleEditor />
				</QueryClientProvider>
			</main>
		</LayoutWrapper>
	);
};

export default Page;
