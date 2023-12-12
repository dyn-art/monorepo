import React from 'react';
import { Button } from '@dyn/ui';

// Styles
import '@dyn/ui/dist/styles.css';
import './styles/tailwind.css';

const App: React.FC = () => {
	return (
		<div>
			<Button>Hello World</Button>
			<div className="bg-blue-300">
				<p className="font-sans font-bold">Jeff</p>
			</div>
		</div>
	);
};

export default App;
