import React from 'react';
import { Input } from '@dyn/ui';

export const DesignTab: React.FC = () => {
	return (
		<>
			<div className="flex flex-row gap-1">
				<Input id="width" placeholder="100" type="number" />
				<Input id="height" placeholder="100" type="number" />
			</div>
			<div>
				<Input id="x" placeholder="100" type="number" />
				<Input id="y" placeholder="100" type="number" />
			</div>
			<div>
				<Input id="angle" placeholder="100" type="number" />
				<Input id="radius" placeholder="100" type="number" />
			</div>
		</>
	);
};
