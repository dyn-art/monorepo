import type { Metadata } from 'next';
import { Inter as FontSans } from 'next/font/google';
import React from 'react';
import { cn } from '@/core/utils';

import './globals.css';

export const fontSans = FontSans({
	subsets: ['latin'],
	variable: '--font-sans'
});

export const metadata: Metadata = {
	title: 'dyn.art',
	description: 'Welcome to dyn.art'
};

const RootLayout: React.FC<TProps> = (props) => {
	const { children } = props;
	return (
		<html lang="en">
			<body className={cn('min-h-screen bg-background font-sans antialiased', fontSans.variable)}>
				{children}
			</body>
		</html>
	);
};

export default RootLayout;

interface TProps {
	children: React.ReactNode;
}
