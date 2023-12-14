import React from 'react';
import { Route, Routes as RouterRoutes } from 'react-router-dom';
import { EAppRoutes } from '@/types';

import { RouteChangeListener } from './components/navigation';
import About from './routes/about';
import Home from './routes/home';

export const Routes: React.FC = () => {
	return (
		<>
			<RouteChangeListener />
			<RouterRoutes>
				<Route path={EAppRoutes.HOME} element={<Home />} />
				<Route path={EAppRoutes.ABOUT} element={<About />} />
			</RouterRoutes>
		</>
	);
};
