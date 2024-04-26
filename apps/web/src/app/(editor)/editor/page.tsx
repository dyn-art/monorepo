import Link from 'next/link';
import React from 'react';

const Page: React.FC = () => {
	return (
		<ul>
			<li>
				<Link href="editor/default">Default</Link>
			</li>
			<li>
				<Link href="editor/square">Sqare</Link>
			</li>
			<li>
				<Link href="editor/m-square">Modifiable Square</Link>
			</li>
			<li>
				<Link href="editor/text">Text</Link>
			</li>
			<li>
				<Link href="editor/screenshot">Screenshot</Link>
			</li>
		</ul>
	);
};

export default Page;
