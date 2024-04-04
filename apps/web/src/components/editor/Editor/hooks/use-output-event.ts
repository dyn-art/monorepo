'use client';

import React from 'react';
import type { Composition, TOutputEventTypeMap, TWatchedOutputEventCallback } from '@dyn/svg-comp';

export function useOutputEvent<GEventType extends keyof TOutputEventTypeMap>(
	composition: Composition,
	eventType: GEventType,
	callback: TWatchedOutputEventCallback<GEventType>,
	callbackDeps: unknown[] = []
): void {
	React.useEffect(() => {
		const unregister = composition.watchOutputEvent(eventType, callback);
		return () => {
			unregister();
		};
	}, [composition, eventType, ...callbackDeps]);
}
