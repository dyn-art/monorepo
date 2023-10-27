import type { ToJsEvent } from '@rust/dyn-dtom/bindings';

import { Composition } from '../core';

(globalThis as any).enqueue_rust_events = function onNewWasmEvents(
	worldId: number,
	events: ToJsEvent[]
) {
	console.log('Received new events from Rust', { worldId, events });
	Composition.onWasmEvents(worldId, events);
};
