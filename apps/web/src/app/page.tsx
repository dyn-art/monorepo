'use client';

import React from 'react';
import { createSVGComposition, initWasm } from '@dyn/dtom';
import { MaxWidthWrapper } from '@/components';

async function onClick(): Promise<void> {
	await initWasm();
	const composition = createSVGComposition({ width: 500, height: 500 });
	const composition2 = createSVGComposition({ width: 500, height: 500 });

	console.log({ composition, composition2 });

	for (let i = 0; i < 10; i++) {
		composition.update();
	}

	console.log('------------------');
	for (let i = 0; i < 10; i++) {
		composition2.update();
	}
}

const Home: React.FC = () => {
	const [isLoading, setIsLoading] = React.useState(false);

	return (
		<MaxWidthWrapper>
			Our WASM component:
			{isLoading ? <div>Loading...</div> : null}
			<button
				onClick={() => {
					setIsLoading(true);
					onClick()
						.catch((err) => {
							console.log(err);
						})
						.finally(() => {
							setIsLoading(false);
						});
				}}
			>
				Load WASM
			</button>
		</MaxWidthWrapper>
	);
};

export default Home;
