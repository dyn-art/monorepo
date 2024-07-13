import type { MetaFunction } from '@remix-run/node';
import React from 'react';

export const meta: MetaFunction = () => {
	return [{ title: 'dyn.art' }, { name: 'description', content: 'Welcome to dyn.art!' }];
};

const Home: React.FC = () => {
	return (
		<main className="relative flex min-h-screen flex-col items-center justify-center bg-[#FEFCF0] p-24">
			<div className="absolute left-0 top-0 ml-12 mt-12 font-mono text-sm">
				<a
					href="https://inbeta.group"
					target="_blank"
					rel="noopener noreferrer"
					className="flex items-center gap-2"
				>
					By <img src="/inbetagroup.svg" alt="inbeta.group Logo" width={157} height={32} />
				</a>
			</div>

			<div className="flex flex-grow items-center justify-center">
				<img src="/dyndotart-blob.svg" alt="dyn.art Logo" width={709} height={625} />
			</div>
		</main>
	);
};

export default Home;
