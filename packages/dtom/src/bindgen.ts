import { Editor } from './Editor';

(globalThis as any).receiveRustEvents = function onNewWasmEvents(worldId: number, data: unknown) {
	Editor.onWasmEvent(worldId, data);
};
