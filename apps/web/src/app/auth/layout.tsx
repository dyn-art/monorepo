import React from 'react';
import { LayoutWrapper } from '@dyn/ui';

const Layout: React.FC<TProps> = (props) => {
	const { children } = props;

	return (
		<LayoutWrapper
			size="full"
			className="flex h-screen flex-col items-center justify-center bg-[#FCFAF4]"
		>
			{children}
		</LayoutWrapper>
	);
};

export default Layout;

interface TProps {
	children: React.ReactNode;
}
