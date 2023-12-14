import React from 'react';
import { Button } from '@dyn/ui';

const About: React.FC = () => {
	return (
		<div>
			<Button>Hello World</Button>
			<p className="text-3xl text-green-500">About</p>
		</div>
	);
};

export default About;
