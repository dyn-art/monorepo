import type { RenderUpdateEvent, SVGAttribute } from '@/rust/dyn_composition_api/bindings';

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
					for (const attribute of creation.attributes) {
						const [key, value] = this.svgAttributeToTuple(attribute);
						newElement.setAttribute(key, value);
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
						if (updateInfo.newValue == null) {
							elementToUpdate.removeAttribute(updateInfo.name);
						} else {
							const [, value] = this.svgAttributeToTuple(updateInfo.newValue);
							elementToUpdate.setAttribute(updateInfo.name, value);
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

	// TODO: refactor to type when changed Rust data type
	private svgAttributeToTuple(attribute: SVGAttribute): [string, string] {
		if ('Id' in attribute) {
			return ['id', attribute.Id.toString()];
		} else if ('Width' in attribute) {
			return ['width', attribute.Width.toString()];
		} else if ('Height' in attribute) {
			return ['height', attribute.Height.toString()];
		} else if ('Opacity' in attribute) {
			return ['opacity', attribute.Opacity.toString()];
		} else if ('Transform' in attribute) {
			const matrix = `matrix(${attribute.Transform.Matrix.join(', ')})`;
			return ['transform', matrix];
		} else if ('D' in attribute) {
			const d = attribute.D.map((command) => {
				if (typeof command === 'object') {
					if ('MoveTo' in command) {
						return `M${command.MoveTo.join(' ')}`;
					} else if ('LineTo' in command) {
						return `L${command.LineTo.join(' ')}`;
					} else if ('CurveTo' in command) {
						return `C${command.CurveTo.join(' ')}`;
					} else if ('ArcTo' in command) {
						return `A${command.ArcTo.join(' ')}`;
					}
				} else if ((command as any) === 'ClosePath') {
					return 'Z';
				}
				return '';
			}).join(' ');
			return ['d', d];
		} else if ('ClipPath' in attribute) {
			return ['clip-path', `url(#${attribute.ClipPath})`];
		} else if ('Fill' in attribute) {
			return ['fill', attribute.Fill];
		} else if ('Name' in attribute) {
			return ['name', attribute.Name];
		}
		return ['', ''];
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
