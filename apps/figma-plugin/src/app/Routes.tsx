import React from 'react';
import { Route, Routes as RouterRoutes } from 'react-router-dom';
import { EAppRoutes } from '@/types';

import { RouteChangeListener } from './components/navigation';
import Home from './routes/home';
import NodeInspectorPlugin from './routes/home.node-inspector';
import ToDTIFPlugin from './routes/home.to-dtif';
import Settings from './routes/settings';

export const Routes: React.FC = () => {
	return (
		<>
			<RouteChangeListener />
			<RouterRoutes>
				<Route path={EAppRoutes.HOME} element={<Home />}>
					<Route path={EAppRoutes.HOME__TO_DTIF} element={<ToDTIFPlugin />} />
					<Route path={EAppRoutes.HOME__NODE_INSPECTOR} element={<NodeInspectorPlugin />} />
				</Route>
				<Route path={EAppRoutes.SETTINGS} element={<Settings />} />
			</RouterRoutes>
		</>
	);
};
