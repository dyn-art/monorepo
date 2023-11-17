import type {
	RenderUpdateEvent,
	SVGAttribute,
	SVGStyle
} from '@/rust/dyn_composition_api/bindings';

import { Renderer } from './Renderer';

export const VERSION = '1.1';
export const NS = 'http://www.w3.org/2000/svg';
export const XLINK = 'http://www.w3.org/1999/xlink';

export class SVGRenderer extends Renderer {
	private _domElement: Element;
	private _svgElement: SVGElement;

	private _svgElementMap = new Map<number, SVGElement>();

	constructor(options: TSVGRendererOptions = {}) {
		super();
		const { domElement = document.body } = options;
		this._domElement = domElement;
		this._svgElement = document.createElementNS(NS, 'svg');
		this._svgElement.setAttribute('version', VERSION);
		this._svgElement.style.setProperty('overflow', 'hidden');
		this._domElement.appendChild(this._svgElement);
	}

	public setSize(width: number, height: number): this {
		this._svgElement.setAttribute('width', `${width}px`);
		this._svgElement.setAttribute('height', `${height}px`);
		return this;
	}

	public render(events: RenderUpdateEvent[]): this {
		for (const renderUpdate of events) {
			const elementId = renderUpdate.id;
			let element: SVGElement | null = null;
			const getElement = (): SVGElement | null => {
				if (element == null) {
					element = this._svgElementMap.get(elementId) ?? null;
				}
				return element;
			};

			for (const update of renderUpdate.updates) {
				switch (update.type) {
					case 'ElementCreated': {
						// Create element
						const newElement: SVGElement = document.createElementNS(NS, update.tagName);
						for (const attribute of update.attributes) {
							const [key, value] = this.parseSVGAttribute(attribute);
							newElement.setAttribute(key, value);
						}
						for (const [key, value] of update.styles as unknown as [string, string][]) {
							newElement.style.setProperty(key, `${value}`);
						}

						this._svgElementMap.set(elementId, newElement);

						// Append element to parent
						if (update.parentId != null) {
							const parentElement = this._svgElementMap.get(update.parentId);
							if (parentElement != null) {
								parentElement.appendChild(newElement);
							}
						} else {
							this._svgElement.appendChild(newElement);
						}

						element = newElement;
						break;
					}
					case 'ElementDeleted': {
						const elementToDelete = getElement();
						if (elementToDelete?.parentNode != null) {
							elementToDelete.parentNode.removeChild(elementToDelete);
							this._svgElementMap.delete(elementId);
						}
						break;
					}
					case 'AttributeUpdated': {
						const elementToUpdate = getElement();
						if (elementToUpdate != null) {
							const [key, value] = this.parseSVGAttribute(update.newValue);
							elementToUpdate.setAttribute(key, value);
						}
						break;
					}
					case 'AttributeRemoved': {
						const elementToUpdate = getElement();
						if (elementToUpdate != null) {
							elementToUpdate.removeAttribute(update.key);
						}
						break;
					}
					case 'StyleUpdated': {
						const elementToUpdate = getElement();
						if (elementToUpdate != null) {
							const [key, value] = this.parseSVGStyle(update.newValue);
							elementToUpdate.style.setProperty(key, value);
						}
						break;
					}
					case 'StyleRemoved': {
						const elementToUpdate = getElement();
						if (elementToUpdate != null) {
							elementToUpdate.style.removeProperty(update.key);
						}
						break;
					}
				}
			}
		}
		return this;
	}

	private parseSVGAttribute(attribute: SVGAttribute): [string, string] {
		switch (attribute.type) {
			case 'Id':
				return ['id', attribute.id.toString()];
			case 'Width':
				return ['width', attribute.width.toString()];
			case 'Height':
				return ['height', attribute.height.toString()];
			case 'Opacity':
				return ['opacity', attribute.opacity.toString()];
			case 'Transform': {
				const transform = attribute.transform;
				const matrix = `matrix(${transform.a}, ${transform.b}, ${transform.c}, ${transform.d}, ${transform.tx}, ${transform.ty})`;
				return ['transform', matrix];
			}
			case 'D': {
				const d = attribute.d
					.map((command) => {
						switch (command.type) {
							case 'MoveTo':
								return `M${command.x} ${command.y}`;
							case 'LineTo':
								return `L${command.x} ${command.y}`;
							case 'CurveTo':
								return `C${command.cx1} ${command.cy1} ${command.cx2} ${command.cy2} ${command.x} ${command.y}`;
							case 'ArcTo':
								return `A${command.rx} ${command.ry} ${command.xAxisRotation} ${
									command.largeArcFlag ? 1 : 0
								} ${command.sweepFlag ? 1 : 0} ${command.x} ${command.y}`;
							case 'ClosePath':
								return 'Z';
							default:
								return '';
						}
					})
					.join(' ');
				return ['d', d];
			}
			case 'ClipPath':
				return ['clip-path', `url(#${attribute.clipPath})`];
			case 'Fill':
				return ['fill', attribute.fill];
			case 'Name':
				return ['name', attribute.name];
			default:
				return ['', ''];
		}
	}

	private parseSVGStyle(style: SVGStyle): [string, string] {
		switch (style.type) {
			case 'Display':
				return ['display', 'block'];
			default:
				return ['', ''];
		}
	}

	public clear(): this {
		while (this._domElement.firstChild) {
			this._domElement.removeChild(this._domElement.firstChild);
		}
		return this;
	}
}

export interface TSVGRendererOptions {
	domElement?: Element;
}