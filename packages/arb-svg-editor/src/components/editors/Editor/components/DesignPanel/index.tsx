import React from 'react';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@dyn/ui';

import { DesignTab } from './DesignTab';

export const DesignPanel: React.FC = () => {
	return (
		<div className="flex h-full p-2 text-sm">
			<Tabs className="w-full" defaultValue="design">
				<TabsList className="grid h-8 w-full grid-cols-2">
					<TabsTrigger className="text-xs" value="design">
						Design
					</TabsTrigger>
					<TabsTrigger className="text-xs" value="automate">
						Automate
					</TabsTrigger>
				</TabsList>
				<TabsContent value="design">
					<DesignTab />
				</TabsContent>
				<TabsContent value="automate">Coming soon</TabsContent>
			</Tabs>
		</div>
	);
};
