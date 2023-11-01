import type { ToJsEvent } from '@/rust/dyn-dtom/bindings';

import { Composition } from '../core';

(globalThis as any).enqueue_rust_events = function onNewWasmEvents(
	worldId: number,
	events: ToJsEvent[]
) {
	Composition.onWasmEvents(worldId, events);
};
