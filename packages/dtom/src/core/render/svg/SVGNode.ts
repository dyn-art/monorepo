import type { Entity } from '@rust/dyn-dtom/bindings';

import {
	createSVGElement,
	removeAttributes,
	setAttributes,
	setStyles,
	type TSVGAttributes,
	type TSVGStyles,
	type TSVGTagNames
} from './dom';

export class SVGNode {
	private readonly _element: SVGElement;
	private _children: Entity[] = [];
	private _parent: Entity | null = null;

	constructor(svgElement: SVGElement) {
		this._element = svgElement;
	}

	// ============================================================================
	// Getter & Setter
	// ============================================================================

	public get element(): SVGElement {
		return this._element;
	}

	public get children(): Entity[] {
		return this._children;
	}

	public get parent(): Entity | null {
		return this._parent;
	}

	// ============================================================================
	// DOM
	// ============================================================================

	public setAttributes(attributes: TSVGAttributes): this {
		setAttributes(this._element, attributes);
		return this;
	}

	public removeAttributes(attributeKeys: string[]): this {
		removeAttributes(this._element, attributeKeys);
		return this;
	}

	public setStyles(styles: TSVGStyles): this {
		setStyles(this._element, styles);
		return this;
	}

	public appendChild(child: SVGNode): this {
		this._element.appendChild(child.element);
		return this;
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

export function createSVGNodeByElement(svgElement: SVGElement): SVGNode {
	return new SVGNode(svgElement);
}

export function createSVGNode(tag: TSVGTagNames, attributes: TSVGAttributes = {}): SVGNode {
	return createSVGNodeByElement(createSVGElement(tag, attributes));
}

export type TEventCallback<TEvent> = (event: TEvent, node: SVGNode) => void;
