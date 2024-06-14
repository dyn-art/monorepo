'use client';

import React from 'react';
import type {
	Canvas,
	TOutputEventTypeMap,
	TWatchedOutputEventCallback
} from '@dyn/cnv-svg-builder';

export function useOutputEvent<GEventType extends keyof TOutputEventTypeMap>(
	canvas: Canvas | null | undefined,
	eventType: GEventType,
	callback: TWatchedOutputEventCallback<GEventType>,
	callbackDeps: unknown[] = []
): void {
	React.useEffect(() => {
		let unregister: (() => void) | null;
		if (canvas != null) {
			unregister = canvas.watchOutputEvent(eventType, callback);
		}
		return () => {
			unregister?.();
		};
	}, [canvas, eventType, ...callbackDeps]);
}
