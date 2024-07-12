import React from 'react';
import { flushSync } from 'react-dom';
import type { ARB } from '@dyn/arb-dtif';
import type { Artboard } from '@dyn/arb-svg-builder';

export function useEntity<GComponentVariants extends ARB.WatchableComponentVariant[]>(
	artboard: Artboard,
	entity: ARB.Entity,
	toWatchComponents: ARB.WatchableComponentVariant[],
	flush = false
): TCombinedComponent<GComponentVariants> {
	const [changes, dispatch] = React.useReducer(changesReducer<GComponentVariants>, {});

	React.useEffect(
		() => {
			const unregister = artboard.watchEntity(
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
		[artboard, entity]
	);

	return changes;
}

function changesReducer<GComponentVariants extends ARB.WatchableComponentVariant[]>(
	state: TCombinedComponent<GComponentVariants>,
	action: ARB.ComponentChange
): TCombinedComponent<GComponentVariants> {
	const { type, ...change } = action;
	return { ...state, [type]: change };
}

export type TCombinedComponent<GComponentVariants extends ARB.WatchableComponentVariant[]> = {
	[K in GComponentVariants[number]]?: TComponent<K>;
};

export type TComponent<GComponentVariant extends ARB.WatchableComponentVariant> = Omit<
	Extract<ARB.ComponentChange, { type: GComponentVariant }>,
	'type'
>;
