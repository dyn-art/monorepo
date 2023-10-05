'use client';

import React from 'react';

async function onClick() {
	const dtom = await import('@dyn/dtom');
	dtom.greetRust();
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
