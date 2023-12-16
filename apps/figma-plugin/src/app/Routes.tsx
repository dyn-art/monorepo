import React from 'react';
import { Route, Routes as RouterRoutes } from 'react-router-dom';
import { EAppRoutes } from '@/types';

import { RouteChangeListener } from './components/navigation';
import Home from './routes/home';
import Settings from './routes/settings';

export const Routes: React.FC = () => {
	return (
		<>
			<RouteChangeListener />
			<RouterRoutes>
				<Route path={EAppRoutes.HOME} element={<Home />} />
				<Route path={EAppRoutes.SETTINGS} element={<Settings />} />
			</RouterRoutes>
		</>
	);
};
