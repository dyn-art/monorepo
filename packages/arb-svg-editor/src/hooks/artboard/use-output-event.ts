'use client';

import React from 'react';
import type {
	Artboard,
	TOutputEventTypeMap,
	TWatchedOutputEventCallback
} from '@dyn/arb-svg-builder';

export function useOutputEvent<GEventType extends keyof TOutputEventTypeMap>(
	artboard: Artboard | null | undefined,
	eventType: GEventType,
	callback: TWatchedOutputEventCallback<GEventType>,
	callbackDeps: unknown[] = []
): void {
	React.useEffect(() => {
		let unregister: (() => void) | null;
		if (artboard != null) {
			unregister = artboard.watchOutputEvent(eventType, callback);
		}
		return () => {
			unregister?.();
		};
	}, [artboard, eventType, ...callbackDeps]);
}
