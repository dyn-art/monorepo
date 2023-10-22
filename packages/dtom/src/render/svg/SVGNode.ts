export class SVGNode {
	private readonly _element: SVGElement;

	constructor(element: SVGElement) {
		this._element = element;
	}

	// ============================================================================
	// Events
	// ============================================================================

	public onWheel(callback: (event: WheelEvent, node: SVGNode) => void) {
		this._element.addEventListener('wheel', (event) => {
			callback(event, this);
		});
	}

	public onPointerDown(callback: (event: PointerEvent, node: SVGNode) => void) {
		this._element.addEventListener('pointerdown', (event) => {
			callback(event, this);
		});
	}

	public onPointerMove(callback: (event: PointerEvent, node: SVGNode) => void) {
		this._element.addEventListener('pointermove', (event) => {
			callback(event, this);
		});
	}

	public onPointerLeave(callback: (event: PointerEvent, node: SVGNode) => void) {
		this._element.addEventListener('pointerleave', (event) => {
			callback(event, this);
		});
	}

	public onPointerUp(callback: (event: PointerEvent, node: SVGNode) => void) {
		this._element.addEventListener('pointerup', (event) => {
			callback(event, this);
		});
	}
}
