export class SVGNode {
	private readonly _element: SVGElement;

	constructor(element: SVGElement) {
		this._element = element;
	}

	// ============================================================================
	// Events
	// ============================================================================

	public onWheel(callback: TEventCallback<WheelEvent>) {
		this._element.addEventListener('wheel', (event) => {
			callback(event, this);
		});
	}

	public onPointerDown(callback: TEventCallback<PointerEvent>) {
		this._element.addEventListener('pointerdown', (event) => {
			callback(event, this);
		});
	}

	public onPointerMove(callback: TEventCallback<PointerEvent>) {
		this._element.addEventListener('pointermove', (event) => {
			callback(event, this);
		});
	}

	public onPointerLeave(callback: TEventCallback<PointerEvent>) {
		this._element.addEventListener('pointerleave', (event) => {
			callback(event, this);
		});
	}

	public onPointerUp(callback: TEventCallback<PointerEvent>) {
		this._element.addEventListener('pointerup', (event) => {
			callback(event, this);
		});
	}
}

export type TEventCallback<TEvent> = (event: TEvent, node: SVGNode) => void;
