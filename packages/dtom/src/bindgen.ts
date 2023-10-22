import { Editor } from './Editor';

(globalThis as any).enqueue_rust_events = function onNewWasmEvents(
	worldId: number,
	events: unknown[]
) {
	console.log('Received new events from Rust', { worldId, events });
	for (const event of events) {
		Editor.onWasmEvent(worldId, event);
	}
};
