import React from 'react';
import {
	Composition,
	Entity,
	MixinChange,
	TrackableMixinType,
	TRustEnumKeyArray
} from '@dyn/svg-composition';

export function useWatchEntity<T extends TTrackableMixinKey[]>(
	composition: Composition,
	entity: Entity,
	toTrackMixinKeys: T
): TCombinedMixin<T> {
	const initialState: TCombinedMixin<T> = {};
	const [changes, dispatch] = React.useReducer(changesReducer<T>, initialState);

	React.useEffect(() => {
		const unwatch = composition.watchEntity(
			entity,
			toTrackMixinKeys,
			(_, changesArray) => {
				for (const change of changesArray) {
					dispatch(change);
				}
			},
			true
		);
		if (!unwatch) {
			console.warn(`Failed to watch Entity: ${entity}!`);
			return;
		}

		return () => {
			unwatch();
		};
	}, [entity, composition]); // Not active effect on toTrackMixinKeys as its an inline array and thus endless loop

	return changes;
}

function changesReducer<T extends TTrackableMixinKey[]>(
	state: TCombinedMixin<T>,
	action: MixinChange
): TCombinedMixin<T> {
	const { type, ...change } = action;
	return { ...state, [type]: change };
}

type TCombinedMixin<T extends TTrackableMixinKey[]> = {
	[K in T[number]]?: Extract<MixinChange, { type: K }>;
};
type TTrackableMixinKey = TRustEnumKeyArray<TrackableMixinType>;
