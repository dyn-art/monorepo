'use client';

import React from 'react';
import type {
	Composition,
	TOutputEventTypeMap,
	TWatchedOutputEventCallback
} from '@dyn/comp-svg-builder';

export function useOutputEvent<GEventType extends keyof TOutputEventTypeMap>(
	composition: Composition | null | undefined,
	eventType: GEventType,
	callback: TWatchedOutputEventCallback<GEventType>,
	callbackDeps: unknown[] = []
): void {
	React.useEffect(() => {
		let unregister: (() => void) | null;
		if (composition != null) {
			unregister = composition.watchOutputEvent(eventType, callback);
		}
		return () => {
			unregister?.();
		};
	}, [composition, eventType, ...callbackDeps]);
}
