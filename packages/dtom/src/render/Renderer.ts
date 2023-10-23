import type { Canvas } from '../canvas';

export abstract class Renderer {
	private _canvas: () => Canvas; // TODO: Bad practice?

	constructor(canvas: Canvas) {
		this._canvas = () => canvas;
	}

	// =========================================================================
	// Getter & Setter
	// =========================================================================

	protected get canvas(): Canvas {
		return this._canvas();
	}

	// =========================================================================
	// Abstract
	// =========================================================================

	public abstract setSize(width: number, height: number): this;

	public abstract render(data: unknown): this;
}
