import React from 'react';
import type { COMP, Composition } from '@dyn/svg-composition';

export function useCursor(composition?: Composition): COMP.CursorForFrontend {
	const [cursor, setCursor] = React.useState<COMP.CursorForFrontend>({
		type: 'Default'
	});

	React.useEffect(() => {
		if (composition) {
			const unwatch = composition.onCursorChange((_cursor) => {
				setCursor(_cursor);
			});
			return () => {
				unwatch();
			};
		}
	}, [composition]);

	return cursor;
}
