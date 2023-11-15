import type { RenderUpdateEvent } from '../../rust_modules/dyn_composition_api/bindings';
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
				if ('ElementCreated' in update) {
					const creation = update.ElementCreated;

					// Create element
					const newElement: SVGElement = document.createElementNS(NS, creation.tagName);
					for (const [key, value] of creation.attributes as unknown as [string, string][]) {
						newElement.setAttribute(key, `${value}`);
					}
					for (const [key, value] of creation.styles as unknown as [string, string][]) {
						newElement.style.setProperty(key, `${value}`);
					}

					this._svgElementMap.set(elementId, newElement);

					// Append element to parent
					if (creation.parentId != null) {
						const parentElement = this._svgElementMap.get(creation.parentId);
						if (parentElement != null) {
							parentElement.appendChild(newElement);
						}
					} else {
						this._svgElement.appendChild(newElement);
					}

					element = newElement;
				} else if ('ElementDeleted' in update) {
					const elementToDelete = getElement();
					if (elementToDelete?.parentNode != null) {
						elementToDelete.parentNode.removeChild(elementToDelete);
						this._svgElementMap.delete(elementId);
					}
				} else if ('AttributeUpdated' in update) {
					const updateInfo = update.AttributeUpdated;
					const elementToUpdate = getElement();
					if (elementToUpdate != null) {
						if (updateInfo.newValue === null) {
							elementToUpdate.removeAttribute(updateInfo.name);
						} else {
							elementToUpdate.setAttribute(updateInfo.name, updateInfo.newValue);
						}
					}
				} else if ('StyleUpdated' in update) {
					const styleUpdate = update.StyleUpdated;
					const elementToUpdate = getElement();
					if (elementToUpdate != null) {
						elementToUpdate.style.setProperty(styleUpdate.name, styleUpdate.newValue || '');
					}
				}
			}
		}
		return this;
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
