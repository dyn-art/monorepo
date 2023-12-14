import React from 'react';
import { Link } from 'react-router-dom';

import { appHandler } from '../app-handler';
import { useAppCallback } from '../hooks';

const Home: React.FC = () => {
	// TODO: REMOVE
	useAppCallback(appHandler, {
		type: 'plugin.message',
		key: 'on-select-frame',
		callback: async (instance, args) => {
			console.log('App: onSelectFrame', { args });
		}
	});
	useAppCallback(appHandler, {
		type: 'plugin.message',
		key: 'on-select-node',
		callback: async (instance, args) => {
			console.log('App: onSelectNode', { args });
		}
	});

	return (
		<div className="m-4 space-y-4">
			<h1 className="text-2xl font-bold">dyn.art</h1>
			<ul className="menu rounded-box bg-base-100 w-56 p-2">
				<li className="menu-title">
					<span>Plugins</span>
				</li>
				<li>
					<Link to="/dtif">Export Frame</Link>
				</li>
				<li>
					<Link to="/node-inspector">Inspect Node</Link>
				</li>
				<li className="menu-title">
					<span>Other</span>
				</li>
				<li>
					<Link to="/about">About</Link>
				</li>
			</ul>
		</div>
	);
};

export default Home;
