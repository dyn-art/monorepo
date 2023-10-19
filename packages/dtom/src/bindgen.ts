import { Editor } from './editor';

(globalThis as any).receiveRustEvents = function onNewWasmEvents(worldId: number, data: unknown) {
	Editor.onWasmEvent(worldId, data);
};
