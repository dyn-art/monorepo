import type {
	Composition,
	Entity,
	MixinChange,
	TrackableMixinType,
	TRustEnumKeyArray
} from '@dyn/svg-composition';
import React from 'react';

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
type TMiddleware<T> = (change: MixinChange) => T;

// TODO: figure out how to solve this as tuple

// interface Example {
// 	a: number;
// 	b: string;
// 	c: boolean;
// }

// const exampleObj: Example = { a: 1, b: 'string', c: true };

// // Optimized MappedTuple type
// type MappedTuple<T, K extends readonly (keyof T)[]> = {
// 	[Index in keyof K]: K[Index] extends keyof T ? T[K[Index]] : never;
// };

// // Improved getValuesFromKeys function
// function getValuesFromKeys<T, K extends readonly (keyof T)[]>(obj: T, keys: K): MappedTuple<T, K> {
// 	return keys.map((key) => obj[key]) as MappedTuple<T, K>;
// }

// // Example usage
// const values = getValuesFromKeys(exampleObj, ['b', 'a'] as const); // values is inferred as [string, number]
