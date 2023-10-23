'use client';

import React from 'react';
import { createSVGEditor, initWasm } from '@dyn/dtom';
import { MaxWidthWrapper } from '@/components';

async function onClick(): Promise<void> {
	await initWasm();
	const editor = createSVGEditor({ width: 500, height: 500 });
	editor.createRect();
	editor.createRect();

	const editor2 = createSVGEditor({ width: 500, height: 500 });
	editor2.createRect();

	console.log({ editor, editor2 });

	for (let i = 0; i < 10; i++) {
		editor.update();
	}

	console.log('------------------');
	for (let i = 0; i < 10; i++) {
		editor2.update();
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
