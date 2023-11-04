import { Outlet } from '@remix-run/react';
import React from 'react';
import { MaxWidthWrapper } from '@/components/layout';

const PlaygroundWrapper: React.FC = () => {
	return (
		<MaxWidthWrapper className="mt-8">
			<Outlet />
		</MaxWidthWrapper>
	);
};

export default PlaygroundWrapper;
