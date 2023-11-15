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
			for (const update of renderUpdate.updates) {
				if ('ElementCreated' in update) {
					const creation = update.ElementCreated;
					const newElement = document.createElementNS(
						'http://www.w3.org/2000/svg',
						creation.tagName
					);
					Object.entries(creation.attributes).forEach(([key, value]) => {
						newElement.setAttribute(key, value);
					});
					Object.entries(creation.styles).forEach(([key, value]) => {
						newElement.style.setProperty(key, value);
					});

					this._svgElementMap.set(elementId, newElement);

					if (creation.parentId != null) {
						const parentElement = this._svgElementMap.get(creation.parentId);
						if (parentElement != null) {
							parentElement.appendChild(newElement);
						}
					}
				} else if ('ElementDeleted' in update) {
					const elementToDelete = this._svgElementMap.get(elementId);
					if (elementToDelete?.parentNode != null) {
						elementToDelete.parentNode.removeChild(elementToDelete);
						this._svgElementMap.delete(elementId);
					}
				} else if ('AttributeUpdated' in update) {
					const updateInfo = update.AttributeUpdated;
					const elementToUpdate = this._svgElementMap.get(elementId);
					if (elementToUpdate != null) {
						if (updateInfo.newValue === null) {
							elementToUpdate.removeAttribute(updateInfo.name);
						} else {
							elementToUpdate.setAttribute(updateInfo.name, updateInfo.newValue);
						}
					}
				} else if ('StyleUpdated' in update) {
					const styleUpdate = update.StyleUpdated;
					const elementToUpdate = this._svgElementMap.get(elementId);
					if (elementToUpdate != null) {
						elementToUpdate.style.setProperty(styleUpdate.name, styleUpdate.newValue || '');
					}
				} else if ('ElementUpdated' in update) {
					const elementUpdate = update.ElementUpdated;
					const elementToUpdate = this._svgElementMap.get(elementId);
					if (elementToUpdate != null) {
						Object.entries(elementUpdate.updatedAttributes).forEach(([key, value]) => {
							elementToUpdate.setAttribute(key, value);
						});
						Object.entries(elementUpdate.updatedStyles).forEach(([key, value]) => {
							elementToUpdate.style.setProperty(key, value);
						});
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
