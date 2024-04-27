import { Analytics } from '@vercel/analytics/react';
import type { Metadata, Viewport } from 'next';
import { Inter as FontSans } from 'next/font/google';
import { cn } from '@dyn/ui';
import { marketingConfig } from '@/environment';

import './globals.scss';

export const fontSans = FontSans({
	subsets: ['latin'],
	variable: '--font-sans'
});

// https://nextjs.org/docs/app/api-reference/functions/generate-viewport
export const viewport: Viewport = {
	themeColor: '#000000',
	width: 'device-width',
	initialScale: 1
};

// https://nextjs.org/docs/app/api-reference/functions/generate-metadata
export const metadata: Metadata = {
	title: marketingConfig.meta.title,
	description: marketingConfig.meta.description,
	icons: [
		{ rel: 'apple-touch-icon', url: '/favicon/apple-touch-icon.png', sizes: '180x180' },
		{ rel: 'icon', url: '/favicon/favicon-32x32.png', sizes: '32x32' },
		{ rel: 'icon', url: '/favicon/favicon-16x16.png', sizes: '16x16' }
	],
	other: {
		'msapplication-TileColor': '#000000',
		'msapplication-config': '/favicon/browserconfig.xml'
	}
};

const RootLayout: React.FC<TProps> = (props) => {
	const { children } = props;
	return (
		<html lang="en">
			<body className={cn('bg-background min-h-screen font-sans antialiased', fontSans.variable)}>
				{children}
				<Analytics />
			</body>
		</html>
	);
};

export default RootLayout;

interface TProps {
	children: React.ReactNode;
}
