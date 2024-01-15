import Image from 'next/image';
import React from 'react';

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
					By{' '}
					<Image src="/inbetagroup.svg" alt="inbeta.group Logo" width={157} height={32} priority />
				</a>
			</div>

			<div className="flex flex-grow items-center justify-center">
				<Image src="/dyndotart-blob.svg" alt="dyn.art Logo" width={709} height={625} priority />
			</div>
		</main>
	);
};

export default Home;
