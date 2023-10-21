import { Editor } from './Editor';

(globalThis as any).receiveRustEvents = function onNewWasmEvents(
	worldId: number,
	events: unknown[]
) {
	for (const event of events) {
		Editor.onWasmEvent(worldId, event);
	}
};
