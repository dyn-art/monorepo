import React from 'react';
import { SVGRender, type Composition } from '@dyn/svg-composition';

export function useForwardEvents<GRef extends Element = HTMLElement>(
	composition: Composition,
	events: string[] = ['wheel']
): React.RefObject<GRef> {
	const sourceRef = React.useRef<GRef>(null);

	React.useEffect(() => {
		if (!sourceRef.current) return;
		const render = composition.renderer[0];
		if (!(render instanceof SVGRender)) return;

		const forwardEvent = (event: Event) => {
			const clonedEvent = new (event.constructor as typeof Event)(event.type, event);
			render.svgElement.dispatchEvent(clonedEvent);
		};

		// Attach event listeners to the source element
		events.forEach((eventType) => {
			sourceRef.current?.addEventListener(eventType, forwardEvent);
		});

		// Cleanup function to remove event listeners
		return () => {
			events.forEach((eventType) => {
				sourceRef.current?.removeEventListener(eventType, forwardEvent);
			});
		};
	}, [sourceRef, composition, events]);

	return sourceRef;
}
