import { enqueue_js_events } from '@rust/dyn-dtom';
import type { FromJsEvent } from '@rust/dyn-dtom/bindings';

export function enqueueJsEvents(worldId: number, events: FromJsEvent[]) {
	enqueue_js_events(worldId, events);
}
