import React from 'react';
import type { Composition, CursorForFrontend } from '@dyn/svg-composition';

export function useCursor(composition?: Composition): CursorForFrontend {
	const [cursor, setCursor] = React.useState<CursorForFrontend>({
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
