import React from 'react';
import { MemoryRouter } from 'react-router-dom';

import '@dyn/ui/dist/styles.css';

import { Routes } from './routes';

import './styles/tailwind.css';

export const App: React.FC = () => {
	return (
		<MemoryRouter>
			<Routes />
		</MemoryRouter>
	);
};
