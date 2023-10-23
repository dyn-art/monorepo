'use client';

import React from 'react';
import { createSVGCanvas, initWasm } from '@dyn/dtom';
import { MaxWidthWrapper } from '@/components';

async function onClick(): Promise<void> {
	await initWasm();
	const canvas = createSVGCanvas({ width: 500, height: 500 });
	canvas.createRect();
	canvas.createRect();

	const canvas2 = createSVGCanvas({ width: 500, height: 500 });
	canvas2.createRect();

	console.log({ canvas, canvas2 });

	for (let i = 0; i < 10; i++) {
		canvas.update();
	}

	console.log('------------------');
	for (let i = 0; i < 10; i++) {
		canvas2.update();
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
