import type { BindgenRenderToJsEvent } from '@/rust/dyn_dtom/bindings';

import { Composition } from '../core';

(globalThis as any).enqueue_rust_events = function onNewWasmEvents(
	worldId: number,
	events: BindgenRenderToJsEvent[]
) {
	Composition.onWasmEvents(worldId, events);
};
