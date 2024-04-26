'use client';

import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import React from 'react';

const queryClient = new QueryClient();

const Layout: React.FC<TProps> = (props) => {
	const { children } = props;

	return <QueryClientProvider client={queryClient}>{children as any}</QueryClientProvider>;
};

export default Layout;

interface TProps {
	children: React.ReactNode;
}
