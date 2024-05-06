import React from 'react';
import type { COMP } from '@dyn/comp-dtif';
import type { Composition } from '@dyn/comp-svg-builder';

import { useOutputEvent } from './use-output-event';

export function useCursor(composition?: Composition): COMP.Cursor {
	const [cursor, setCursor] = React.useState<COMP.Cursor>({
		type: 'Default'
	});
	useOutputEvent(
		composition,
		'CursorChange',
		(event) => {
			setCursor(event.cursor);
		},
		[]
	);
	return cursor;
}
