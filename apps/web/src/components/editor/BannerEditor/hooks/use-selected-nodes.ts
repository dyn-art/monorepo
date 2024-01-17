import React from 'react';
import type { Composition, Entity } from '@dyn/svg-composition';

export function useSelectedNodes(composition: Composition): Entity[] {
	const [selectedEntities, setSelectedEntities] = React.useState<Entity[]>([]);

	React.useEffect(() => {
		const unwatch = composition.onSelectionChange((selected) => {
			setSelectedEntities(selected);
		});
		return () => {
			unwatch();
		};
	}, [composition]);

	return selectedEntities;
}
