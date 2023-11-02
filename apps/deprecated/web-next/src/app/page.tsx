import Image from 'next/image';
import React from 'react';

const Home: React.FC = () => {
	return (
		<main className="relative min-h-screen flex flex-col items-center justify-center p-24 bg-[#FEFCF0]">
			<div className="absolute top-0 left-0 mt-12 ml-12 font-mono text-sm">
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

			<div className="flex-grow flex items-center justify-center">
				<Image src="/dyndotart-blob.svg" alt="dyn.art Logo" width={709} height={625} priority />
			</div>
		</main>
	);
};

export default Home;
