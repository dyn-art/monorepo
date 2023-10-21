import { Renderer, type TRendererOptions } from './Renderer';

export class SVGRenderer extends Renderer {
	private _domElement: SVGElement;
	private _defsElement: SVGElement;

	private readonly _version = '1.1';
	private readonly _ns = 'http://www.w3.org/2000/svg';
	private readonly _xlink = 'http://www.w3.org/1999/xlink';

	constructor(options: TSVGRendererOptions = {}) {
		super(options);
		const { domElement = this.createSVGElement('svg') } = options;
		this._domElement = domElement;
		this._defsElement = this.createSVGElement('defs');
		this._domElement.appendChild(this._defsElement);
		this._domElement.style.overflow = 'hidden';
	}

	public setSize(width: number, height: number): this {
		this._width = width;
		this._height = height;
		this.setAttributes(this._domElement, { width: `${width}px`, height: `${height}px` });
		// TODO: trigger resize event
		return this;
	}

	public render(data: unknown): this {
		// TODO:
		console.log('SVG render', { data });
		return this;
	}

	// Create a namespaced SVG element
	private createSVGElement(tag: TSVGTagNames, attributes: Record<string, string> = {}): SVGElement {
		const element = document.createElementNS(this._ns, tag);
		if (tag === 'svg' && attributes.version == null) {
			attributes.version = this._version;
		}
		if (Object.keys(attributes).length > 0) {
			this.setAttributes(element, attributes);
		}
		return element;
	}

	// Add attributes to SVG element
	private setAttributes(element: SVGElement, attributes: Record<string, string>): this {
		for (const [key, value] of Object.entries(attributes)) {
			if (key.includes('href')) {
				element.setAttributeNS(this._xlink, key, value);
			} else {
				element.setAttribute(key, value);
			}
		}
		return this;
	}

	// Remove attributes from SVG element
	private removeAttributes(element: Element, attributeKeys: string[]): this {
		for (const attr of attributeKeys) {
			element.removeAttribute(attr);
		}
		return this;
	}
}

export type TSVGRendererOptions = {
	domElement?: SVGElement;
} & TRendererOptions;

export type TSVGTagNames =
	| 'svg'
	| 'circle'
	| 'ellipse'
	| 'line'
	| 'path'
	| 'polygon'
	| 'polyline'
	| 'rect'
	| 'text'
	| 'use'
	| 'defs'
	| 'g'
	| 'symbol';
