import React from 'react';
import { flushSync } from 'react-dom';
import type { CNV } from '@dyn/cnv-dtif';
import type { Canvas } from '@dyn/cnv-svg-builder';

export function useEntity<GComponentVariants extends CNV.WatchableComponentVariant[]>(
	canvas: Canvas,
	entity: CNV.Entity,
	toWatchComponents: CNV.WatchableComponentVariant[],
	flush = false
): TCombinedComponent<GComponentVariants> {
	const [changes, dispatch] = React.useReducer(changesReducer<GComponentVariants>, {});

	React.useEffect(
		() => {
			const unregister = canvas.watchEntity(
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
		[canvas, entity]
	);

	return changes;
}

function changesReducer<GComponentVariants extends CNV.WatchableComponentVariant[]>(
	state: TCombinedComponent<GComponentVariants>,
	action: CNV.ComponentChange
): TCombinedComponent<GComponentVariants> {
	const { type, ...change } = action;
	return { ...state, [type]: change };
}

export type TCombinedComponent<GComponentVariants extends CNV.WatchableComponentVariant[]> = {
	[K in GComponentVariants[number]]?: TComponent<K>;
};

export type TComponent<GComponentVariant extends CNV.WatchableComponentVariant> = Omit<
	Extract<CNV.ComponentChange, { type: GComponentVariant }>,
	'type'
>;
