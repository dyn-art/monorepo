export class FigmaAppHandler {
	private readonly _parent: Window;

	constructor(parentInstance: Window) {
		this._parent = parentInstance;

		// figma.ui.onmessage = (message) => {
		// 	// TODO
		// };

		// figma.on('');

		// figma.ui.postMessage('');
	}

	public get parent() {
		return this._parent;
	}
}
