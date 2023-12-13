export class FigmaPluginHandler {
	private readonly _figma: typeof figma;

	constructor(figmaInstance: typeof figma) {
		this._figma = figmaInstance;
	}

	public get figma() {
		return this._figma;
	}
}
