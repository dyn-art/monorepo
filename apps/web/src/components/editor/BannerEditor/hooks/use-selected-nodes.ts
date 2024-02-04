import React from 'react';
import type { COMP, Composition } from '@dyn/svg-composition';

export function useSelectedNodes(composition: Composition): COMP.Entity[] {
	const [selectedEntities, setSelectedEntities] = React.useState<COMP.Entity[]>([]);

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
