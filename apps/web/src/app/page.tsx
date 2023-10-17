'use client';

import React from 'react';
import * as dtom from '@dyn/dtom';
import { MaxWidthWrapper } from '@/components';

async function onClick(): Promise<void> {
	await dtom.initWasm();
	const editor = dtom.editorFactory();
	editor.createRect();
	editor.createRect();

	console.log(editor);

	for (let i = 0; i < 10; i++) {
		editor.update();
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
