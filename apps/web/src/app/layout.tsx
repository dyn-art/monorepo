import { Analytics } from '@vercel/analytics/react';
import type { Metadata } from 'next';
import { Alegreya_Sans, Lato } from 'next/font/google';
import { cn, ScreenSize } from '@dyn/ui';
import { appConfig, marketingConfig } from '@/environment';

import './globals.css';

export const fontSans = Lato({
	subsets: ['latin'],
	variable: '--font-sans',
	weight: '400'
});

export const fontBody = Lato({
	subsets: ['latin'],
	variable: '--font-body',
	weight: '400'
});

export const fontDisplay = Alegreya_Sans({
	subsets: ['latin'],
	variable: '--font-display',
	weight: '400'
});

// https://nextjs.org/docs/app/api-reference/functions/generate-metadata
export const metadata: Metadata = {
	metadataBase: new URL(appConfig.url),
	title: {
		default: marketingConfig.meta.title.default,
		template: marketingConfig.meta.title.template('%s')
	},
	description: marketingConfig.meta.description
};

const Layout: React.FC<TProps> = (props) => {
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

export default Layout;

interface TProps {
	children: React.ReactNode;
}
