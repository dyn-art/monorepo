import React from 'react';
import type { ARB } from '@dyn/arb-dtif';
import type { Artboard } from '@dyn/arb-svg-builder';

import { useOutputEvent } from '../use-output-event';

export function useCursor(artboart?: Artboard): ARB.Cursor {
	const [cursor, setCursor] = React.useState<ARB.Cursor>({
		type: 'Default'
	});
	useOutputEvent(
		artboart,
		'CursorChange',
		(event) => {
			setCursor(event.cursor);
		},
		[]
	);
	return cursor;
}
