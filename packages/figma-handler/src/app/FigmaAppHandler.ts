export class FigmaAppHandler {
	private readonly _parent: Window;

	constructor(parentInstance: Window) {
		this._parent = parentInstance;
	}

	public get parent() {
		return this._parent;
	}
}
