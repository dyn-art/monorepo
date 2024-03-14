import React from 'react';
import type { COMP } from '@dyn/comp-dtif';
import type { Composition } from '@dyn/svg-comp';

export function useEntity<GComponentVariants extends COMP.WatchableComponentVariant[]>(
	composition: Composition,
	entity: COMP.Entity,
	toWatchComponents: COMP.WatchableComponentVariant[]
): TCombinedComponent<GComponentVariants> {
	const [changes, dispatch] = React.useReducer(changesReducer<GComponentVariants>, {});

	React.useEffect(
		() => {
			const unregister = composition.watchEntity(
				entity,
				toWatchComponents,
				(_, changesArray) => {
					for (const change of changesArray) {
						dispatch(change);
					}
				},
				true
			);

			return () => {
				unregister();
			};
		},
		// Not taking "toWatchComponents" in consideration
		// because its most likely an inline array and thus the pointer changes every render
		[composition, entity]
	);

	return changes;
}

function changesReducer<GComponentVariants extends COMP.WatchableComponentVariant[]>(
	state: TCombinedComponent<GComponentVariants>,
	action: COMP.ComponentChange
): TCombinedComponent<GComponentVariants> {
	const { type, ...change } = action;
	return { ...state, [type]: change };
}

type TCombinedComponent<GComponentVariants extends COMP.WatchableComponentVariant[]> = {
	[K in GComponentVariants[number]]?: Omit<Extract<COMP.ComponentChange, { type: K }>, 'type'>;
};
