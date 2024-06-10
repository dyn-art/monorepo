import { Analytics } from '@vercel/analytics/react';
import type { Metadata, Viewport } from 'next';
import { Alegreya_Sans, Lato } from 'next/font/google';
import { cn, ScreenSize } from '@dyn/ui';
import { appConfig, marketingConfig } from '@/environment';

import './globals.scss';

export const fontSans = Lato({
	subsets: ['latin'],
	variable: '--font-sans',
	weight: '400'
});

export const fontBody = Lato({
	subsets: ['latin'],
	variable: '--font-sans',
	weight: '400'
});

export const fontDisplay = Alegreya_Sans({
	subsets: ['latin'],
	variable: '--font-display',
	weight: '400'
});

// https://nextjs.org/docs/app/api-reference/functions/generate-viewport
export const viewport: Viewport = {
	themeColor: '#000000',
	width: 'device-width',
	initialScale: 1
};

// https://nextjs.org/docs/app/api-reference/functions/generate-metadata
export const metadata: Metadata = {
	metadataBase: new URL(appConfig.url),
	title: {
		default: marketingConfig.meta.title.default,
		template: marketingConfig.meta.title.template
	},
	description: marketingConfig.meta.description,
	openGraph: {
		title: marketingConfig.meta.title,
		description: 'todo',
		url: appConfig.url,
		siteName: marketingConfig.meta.description,
		locale: 'en_US',
		type: 'website'
	},
	robots: {
		index: true,
		follow: true,
		googleBot: {
			'index': true,
			'follow': true,
			'max-video-preview': -1,
			'max-image-preview': 'large',
			'max-snippet': -1
		}
	},
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
			<body
				className={cn(
					'bg-background min-h-screen font-sans antialiased',
					fontSans.variable,
					fontBody.variable,
					fontDisplay.variable
				)}
			>
				{children}
				<Analytics />
				{appConfig.url.includes('localhost') && <ScreenSize />}
			</body>
		</html>
	);
};

export default RootLayout;

interface TProps {
	children: React.ReactNode;
}
