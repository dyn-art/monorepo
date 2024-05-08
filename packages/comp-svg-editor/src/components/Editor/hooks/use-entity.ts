import React from 'react';
import { flushSync } from 'react-dom';
import type { COMP } from '@dyn/comp-dtif';
import type { Composition } from '@dyn/comp-svg-builder';

export function useEntity<GComponentVariants extends COMP.WatchableComponentVariant[]>(
	composition: Composition,
	entity: COMP.Entity,
	toWatchComponents: COMP.WatchableComponentVariant[],
	flush = false
): TCombinedComponent<GComponentVariants> {
	const [changes, dispatch] = React.useReducer(changesReducer<GComponentVariants>, {});

	React.useEffect(
		() => {
			const unregister = composition.watchEntity(
				entity,
				toWatchComponents,
				(_, changesArray) => {
					if (flush) {
						// TODO: Validate that flushSync() is a good idea here
						// setTimeout(() => {
						flushSync(() => {
							for (const change of changesArray) {
								dispatch(change);
							}
						});
						// });
					} else {
						for (const change of changesArray) {
							dispatch(change);
						}
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

export type TCombinedComponent<GComponentVariants extends COMP.WatchableComponentVariant[]> = {
	[K in GComponentVariants[number]]?: TComponent<K>;
};

export type TComponent<GComponentVariant extends COMP.WatchableComponentVariant> = Omit<
	Extract<COMP.ComponentChange, { type: GComponentVariant }>,
	'type'
>;
