import { Link } from '@remix-run/react';
import React from 'react';
import { Separator } from '@/components/primitive';

const Playground: React.FC = () => {
	return (
		<>
			<div className="space-y-1">
				<h4 className="text-sm font-medium leading-none">Playground</h4>
				<p className="text-sm text-muted-foreground">Test and compare different canvas engines.</p>
			</div>
			<Separator className="my-4" />
			<div className="flex h-5 items-center space-x-4 text-sm">
				<Link to={'/playground/dtom'}>DTOM</Link>
				<Separator orientation="vertical" />
				<Link to={'/playground/twojs'}>Two.js</Link>
			</div>
		</>
	);
};

export default Playground;
