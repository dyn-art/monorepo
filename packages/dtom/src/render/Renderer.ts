export abstract class Renderer {
	protected _width: number;
	protected _height: number;

	constructor(options: TRendererOptions = {}) {
		const { width = 100, height = 100 } = options;
		this._width = width;
		this._height = height;
	}

	public abstract setSize(width: number, height: number): this;

	public abstract render(): this;
}

export interface TRendererOptions {
	width?: number;
	height?: number;
}
