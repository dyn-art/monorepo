import React from 'react';

const Page: React.FC<TProps> = (props) => {
	const {
		params: { id }
	} = props;

	return (
		<div>
			<p>Hello World {id}</p>
		</div>
	);
};

export default Page;

interface TProps {
	params: { id: string };
}
