import { Button } from '@dyn/ui';

import { Composition } from './Composition';

const Page: React.FC = () => {
	return (
		<div>
			<p>Hello App</p>
			<Button>Hello World</Button>
			<Composition />
		</div>
	);
};

export default Page;
