import React from 'react';
import { MemoryRouter } from 'react-router-dom';

import '@dyn/ui/dist/styles.css';

import { Routes } from './routes';

import './styles/tailwind.css';

export const App: React.FC = () => {
	return (
		<div className="flex h-full w-full flex-col bg-white py-2">
			<MemoryRouter>
				<Routes />
			</MemoryRouter>
		</div>
	);
};
