import React from 'react';
import { Outlet, useLocation, useNavigate } from 'react-router-dom';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@dyn/ui';

import { EAppRoutes } from '../../types';
import { Footer, Navbar } from '../components';

const Home: React.FC = () => {
	const navigate = useNavigate();
	const location = useLocation();
	const defaultPluginIndex = React.useMemo(
		() =>
			plugins.findIndex(
				(plugin) => `${EAppRoutes.HOME}${plugin.route.toString()}` === location.pathname
			),
		[location.pathname]
	);

	// Navigate to default plugin if its the home ("/") route
	React.useEffect(() => {
		if (location.pathname === EAppRoutes.HOME.toString()) {
			navigate(EAppRoutes.HOME__TO_DTIF);
		}
	}, [location.pathname]);

	return (
		<>
			<Navbar
				leftContent={
					<Select
						value={defaultPluginIndex.toString()}
						onValueChange={(index: string) => {
							const route = plugins[Number(index)]?.route;
							if (route != null) {
								navigate(route);
							}
						}}
					>
						<SelectTrigger className={'h-7 max-w-[200px] text-xs [&_svg]:h-4 [&_svg]:w-4'}>
							<span className="text-muted-foreground mr-1">Plugin: </span>
							<SelectValue placeholder="Select plugin" />
						</SelectTrigger>
						<SelectContent>
							{plugins.map((plugin, index) => (
								<SelectItem key={plugin.key} value={index.toString()} className="text-xs">
									{plugin.label}
								</SelectItem>
							))}
						</SelectContent>
					</Select>
				}
				rightContent={{ variant: 'user' }}
			/>

			<Outlet />

			<Footer leftContent={{ variant: 'version' }} rightContent={{ variant: 'settings' }} />
		</>
	);
};

export default Home;

export const plugins = [
	{
		key: 'to-dtif',
		label: 'To DTIF',
		route: EAppRoutes.HOME__TO_DTIF
	},
	{
		key: 'node-inspector',
		label: 'Node Inspector',
		route: EAppRoutes.HOME__NODE_INSPECTOR
	}
] as const;

export type Plugin = (typeof plugins)[number];
