import React from 'react';
import type { Composition, TOutputEventTypeMap } from '@dyn/svg-comp';

export function useOutputEvent<GEventType extends keyof TOutputEventTypeMap>(
	comp: Composition,
	eventType: GEventType
): TOutputEventTypeMap[GEventType] | null {
	const [eventValue, setEventValue] = React.useState<TOutputEventTypeMap[GEventType] | null>(null);

	React.useEffect(() => {
		const unregister = comp.watchOutputEvent(eventType, setEventValue);
		return () => {
			unregister();
		};
	}, [comp, eventType]);

	return eventValue;
}
