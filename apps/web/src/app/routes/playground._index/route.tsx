import { Link } from '@remix-run/react';
import React from 'react';
import { MaxWidthWrapper } from '@/components/layout';
import { Separator } from '@/components/primitive';

const Playground: React.FC = () => {
	return (
		<MaxWidthWrapper className="mt-8">
			<div className="space-y-1">
				<h4 className="text-sm font-medium leading-none">Playground</h4>
				<p className="text-muted-foreground text-sm">Test and compare different canvas engines.</p>
			</div>
			<Separator className="my-4" />
			<div className="flex h-5 items-center space-x-4 text-sm">
				<Link to={'/playground/dtom'}>DTOM</Link>
				<Separator orientation="vertical" />
				<Link to={'/playground/twojs'}>Two.js</Link>
			</div>
		</MaxWidthWrapper>
	);
};

export default Playground;
