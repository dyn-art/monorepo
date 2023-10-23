import type { Editor } from '../editor';

export abstract class Renderer {
	private _editor: () => Editor; // TODO: Bad practice?

	constructor(editor: Editor) {
		this._editor = () => editor;
	}

	// =========================================================================
	// Getter & Setter
	// =========================================================================

	protected get editor(): Editor {
		return this._editor();
	}

	// =========================================================================
	// Abstract
	// =========================================================================

	public abstract setSize(width: number, height: number): this;

	public abstract render(data: unknown): this;
}
