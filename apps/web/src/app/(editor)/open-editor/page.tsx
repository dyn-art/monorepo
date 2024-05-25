import Link from 'next/link';
import React from 'react';

const Page: React.FC = () => {
	return (
		<ul>
			<li>
				<Link href="open-editor/default">Default</Link>
			</li>
			<li>
				<Link href="open-editor/tweet">Tweet</Link>
			</li>
			<li>
				<Link href="open-editor/m-tweet">Modifiable Tweet</Link>
			</li>
			<li>
				<Link href="open-editor/square">Sqare</Link>
			</li>
			<li>
				<Link href="open-editor/m-square">Modifiable Square</Link>
			</li>
			<li>
				<Link href="open-editor/text">Text</Link>
			</li>
			<li>
				<Link href="open-editor/nested">Nested</Link>
			</li>
			<li>
				<Link href="open-editor/constraints">Constraints</Link>
			</li>
		</ul>
	);
};

export default Page;
