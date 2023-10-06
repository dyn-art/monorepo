'use client';

import React from 'react';
import * as dtom from '@dyn/dtom';

async function onClick() {
	const result = await dtom.greetRust();
	console.log(result);
}

export default function Page(): JSX.Element {
	const [isLoading, setIsLoading] = React.useState(false);

	return (
		<div>
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
		</div>
	);
}
