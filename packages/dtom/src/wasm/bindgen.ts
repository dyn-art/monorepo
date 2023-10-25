import type { ToJsEvent } from '@rust/dyn-dtom/bindings';

import { Composition } from '../composition';

(globalThis as any).enqueue_rust_events = function onNewWasmEvents(
	worldId: number,
	events: ToJsEvent[]
) {
	console.log('Received new events from Rust', { worldId, events });
	for (const event of events) {
		Composition.onWasmEvent(worldId, event);
	}
};
