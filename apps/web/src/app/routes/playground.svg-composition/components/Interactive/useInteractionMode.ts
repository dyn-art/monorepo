import React from 'react';
import { Composition, RawInteractionMode, TRustEnumKeyArray } from '@dyn/svg-composition';

export function useInteractionMode(
	composition: Composition
): TRustEnumKeyArray<RawInteractionMode> {
	const [interactionMode, setInteractionMode] =
		React.useState<TRustEnumKeyArray<RawInteractionMode>>('None');

	React.useEffect(() => {
		const unwatch = composition.onInteractionModeChange((interactionMode) => {
			setInteractionMode(interactionMode);
		});
		return () => {
			unwatch();
		};
	}, [composition]);

	return interactionMode;
}
