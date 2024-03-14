import React from 'react';
import type { Composition, TOutputEventTypeMap } from '@dyn/svg-comp';

export function useOutputEvent<GEventType extends keyof TOutputEventTypeMap>(
	composition: Composition,
	eventType: GEventType
): TOutputEventTypeMap[GEventType] | null {
	const [eventValue, setEventValue] = React.useState<TOutputEventTypeMap[GEventType] | null>(null);

	React.useEffect(() => {
		const unregister = composition.watchOutputEvent(eventType, setEventValue);
		return () => {
			unregister();
		};
	}, [composition, eventType]);

	return eventValue;
}
