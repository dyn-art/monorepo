import type {
	RenderUpdateEvent,
	SVGAttribute,
	SVGStyle
} from '@/rust/dyn_composition_api/bindings';

import type { Composition } from '../composition';
import { Renderer } from './Renderer';

export const VERSION = '1.1';
export const NS = 'http://www.w3.org/2000/svg';
export const XLINK = 'http://www.w3.org/1999/xlink';

export class SVGRenderer extends Renderer {
	private _domElement: Element;
	private _svgElement: SVGElement;

	private _svgElementMap = new Map<number, SVGElement>();

	constructor(composition: Composition, options: TSVGRendererOptions = {}) {
		super(composition);
		const { domElement = document.body } = options;
		this._domElement = domElement;
		this._svgElement = document.createElementNS(NS, 'svg');
		this._svgElement.setAttribute('version', VERSION);
		this._svgElement.style.setProperty('overflow', 'hidden');
		this._domElement.appendChild(this._svgElement);
	}

	public setSize(width: number, height: number): void {
		this._svgElement.setAttribute('width', `${width}px`);
		this._svgElement.setAttribute('height', `${height}px`);
	}

	public render(events: RenderUpdateEvent[]): void {
		for (const renderUpdate of events) {
			let element: SVGElement | null = null;
			const getElement = (): SVGElement | null => {
				if (element == null) {
					element = this._svgElementMap.get(renderUpdate.id) ?? null;
				}
				return element;
			};

			for (const update of renderUpdate.updates) {
				switch (update.type) {
					case 'ElementCreated': {
						const newElement: SVGElement = document.createElementNS(NS, update.tagName);

						// Apply attributes
						for (const attribute of update.attributes) {
							const parsedAttribute = this.parseSVGAttribute(attribute);
							if (parsedAttribute != null) {
								const [key, value] = parsedAttribute;
								newElement.setAttribute(key, value);
							}
						}

						// Apply styles
						for (const style of update.styles) {
							const parsedStyle = this.parseSVGStyle(style);
							if (parsedStyle != null) {
								const [key, value] = parsedStyle;
								newElement.style.setProperty(key, `${value}`);
							}
						}

						// Register callbacks
						if (update.isBundleRoot) {
							newElement.addEventListener('pointerdown', () => {
								this.composition.emitInteractionEvents([
									{ type: 'CursorDownOnEntity', entity: renderUpdate.id }
								]);
							});
						}

						this._svgElementMap.set(renderUpdate.id, newElement);

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
							this._svgElementMap.delete(renderUpdate.id);
						}
						break;
					}
					case 'ElementAppended': {
						const toAppendElement = getElement();
						const parentElement = this._svgElementMap.get(renderUpdate.id);
						if (parentElement != null && toAppendElement != null) {
							parentElement.appendChild(toAppendElement);
						}
						break;
					}
					case 'AttributeUpdated': {
						const elementToUpdate = getElement();
						if (elementToUpdate != null) {
							const parsedAttribute = this.parseSVGAttribute(update.newValue);
							if (parsedAttribute != null) {
								const [key, value] = parsedAttribute;
								elementToUpdate.setAttribute(key, value);
							}
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
							const parsedStyle = this.parseSVGStyle(update.newValue);
							if (parsedStyle != null) {
								const [key, value] = parsedStyle;
								elementToUpdate.style.setProperty(key, value);
							}
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
	}

	private parseSVGAttribute(attribute: SVGAttribute): [string, string] | null {
		switch (attribute.type) {
			case 'Id':
				return ['id', attribute.id.toString()];
			case 'Width':
				switch (attribute.unit.type) {
					case 'Pixel':
						return ['width', attribute.width.toString()];
					case 'Percent':
						return ['width', `${attribute.width}%`];
					default:
						return null;
				}
			case 'Height':
				switch (attribute.unit.type) {
					case 'Pixel':
						return ['height', attribute.height.toString()];
					case 'Percent':
						return ['height', `${attribute.height}%`];
					default:
						return null;
				}
			case 'Opacity':
				return ['opacity', attribute.opacity.toString()];
			case 'Transform': {
				const transform = attribute.transform;
				switch (transform.type) {
					case 'Matrix':
						return [
							'transform',
							`matrix(${transform.a}, ${transform.b}, ${transform.c}, ${transform.d}, ${transform.tx}, ${transform.ty})`
						];
					default:
						return ['', ''];
				}
			}
			case 'D':
				return [
					'd',
					attribute.d
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
						.join(' ')
				];
			case 'ClipPath':
				return ['clip-path', `url(#${attribute.clipPath})`];
			case 'Fill':
				return ['fill', attribute.fill];
			case 'Name':
				return ['name', attribute.name];
			default:
				return null;
		}
	}

	private parseSVGStyle(style: SVGStyle): [string, string] | null {
		switch (style.type) {
			case 'Display': {
				const display = style.display;
				switch (display.type) {
					case 'Block':
						return ['display', 'block'];
					case 'None':
						return ['display', 'none'];
					default:
						return null;
				}
			}
			case 'BlendMode': {
				const blendMode = style.blendMode;
				switch (blendMode.type) {
					case 'Normal':
						return null; // 'mix-blend-mode: normal' is the default
					case 'Multiply':
						return ['mix-blend-mode', 'multiply'];
					case 'Screen':
						return ['mix-blend-mode', 'screen'];
					case 'Overlay':
						return ['mix-blend-mode', 'overlay'];
					case 'Darken':
						return ['mix-blend-mode', 'darken'];
					case 'Lighten':
						return ['mix-blend-mode', 'lighten'];
					case 'ColorDodge':
						return ['mix-blend-mode', 'color-dodge'];
					case 'ColorBurn':
						return ['mix-blend-mode', 'color-burn'];
					case 'HardLight':
						return ['mix-blend-mode', 'hard-light'];
					case 'SoftLight':
						return ['mix-blend-mode', 'soft-light'];
					case 'Difference':
						return ['mix-blend-mode', 'difference'];
					case 'Exclusion':
						return ['mix-blend-mode', 'exclusion'];
					case 'Hue':
						return ['mix-blend-mode', 'hue'];
					case 'Saturation':
						return ['mix-blend-mode', 'saturation'];
					case 'Color':
						return ['mix-blend-mode', 'color'];
					case 'Luminosity':
						return ['mix-blend-mode', 'luminosity'];
					default:
						return null;
				}
			}
			default:
				return null;
		}
	}

	public clear(): void {
		while (this._domElement.firstChild) {
			this._domElement.removeChild(this._domElement.firstChild);
		}
	}
}

export interface TSVGRendererOptions {
	domElement?: Element;
}
