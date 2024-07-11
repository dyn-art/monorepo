import React from 'react';
import { Composition, CursorForFrontend } from '@dyn/svg-composition';

export function useCursor(composition?: Composition): CursorForFrontend {
	const [cursor, setCursor] = React.useState<CursorForFrontend>({
		type: 'Default'
	});

	React.useEffect(() => {
		if (composition) {
			const unwatch = composition.onCursorChange((cursor) => {
				setCursor(cursor);
			});
			return () => {
				unwatch();
			};
		}
	}, [composition]);

	return cursor;
}
