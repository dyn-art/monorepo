import { toMouseButton } from '@dyn/dtif';
import type {
	CompositionChange,
	ElementChangeEvent,
	SVGAttribute,
	SVGStyle,
	Vec2
} from '@/rust/dyn_svg_composition_api/bindings';

import type { Composition } from '../composition';
import { Render } from './Render';

export const VERSION = '1.1';
export const NS = 'http://www.w3.org/2000/svg';
export const XLINK = 'http://www.w3.org/1999/xlink';

export class SVGRender extends Render {
	private _domElement: Element;
	private _svgElement: SVGElement;

	private _svgElementMap = new Map<number, SVGElement>();

	private _isCursorOutOfComposion = true;

	private _viewBox: { width: number; height: number };
	private _width: number;
	private _height: number;

	constructor(composition: Composition, options: TSVGRendererOptions = {}) {
		super(composition);
		const { domElement = document.body } = options;
		this._domElement = domElement;

		// Create SVG root
		this._svgElement = document.createElementNS(NS, 'svg');
		this._svgElement.setAttribute('version', VERSION);
		this._svgElement.style.setProperty('overflow', 'hidden');
		this._domElement.appendChild(this._svgElement);

		// Register SVG root callbacks
		this._svgElement.addEventListener('pointermove', (e) => {
			e.preventDefault();
			this.composition.emitInteractionEvents(
				[
					{
						type: 'CursorMovedOnComposition',
						position: this.pointerEventToCompositionPoint(e)
					}
				],
				false
			);
		});
		this._svgElement.addEventListener('wheel', (e) => {
			e.preventDefault();
			this.composition.emitInteractionEvents(
				[
					{
						type: 'WheeledOnComposition',
						position: this.clientWindowPointToCompositionPoint([e.clientX, e.clientY]),
						ctrlKeyPressed: e.ctrlKey,
						metaKeyPressed: e.metaKey,
						delta: [e.deltaX, e.deltaY]
					}
				],
				false
			);
		});
		this._svgElement.addEventListener('pointerdown', (e) => {
			e.preventDefault();
			this.composition.emitInteractionEvents([
				{
					type: 'CursorDownOnComposition',
					position: this.pointerEventToCompositionPoint(e),
					button: toMouseButton(e.button)
				}
			]);
		});
		this._svgElement.addEventListener('pointerup', (e) => {
			this.composition.emitInteractionEvents([
				{
					type: 'CursorUpOnComposition',
					position: this.pointerEventToCompositionPoint(e),
					button: toMouseButton(e.button)
				}
			]);
		});
		this._svgElement.addEventListener('pointerenter', () => {
			if (this._isCursorOutOfComposion) {
				this.composition.emitInteractionEvents([{ type: 'CursorEnteredComposition' }]);
				this._isCursorOutOfComposion = false;
			}
		});
		this._svgElement.addEventListener('pointerleave', (e) => {
			const compositionPoint = this.pointerEventToCompositionPoint(e);
			// Check whether cursor actually left composition
			// or whether its just on some UI layer like the selection box
			if (compositionPoint[0] < 0 || compositionPoint[1] < 0) {
				this.composition.emitInteractionEvents([{ type: 'CursorExitedComposition' }]);
				this._isCursorOutOfComposion = true;
			}
		});
	}

	public get svgElement(): Readonly<SVGElement> {
		return this._svgElement;
	}

	public setViewBox(width: number, height: number): void {
		this._viewBox = { width, height };
		this._svgElement.setAttribute('viewBox', `0 0 ${width} ${height}`);
	}

	public applyCompositionChange(change: CompositionChange): void {
		this.renderComposition(change);
	}

	public applyElementChanges(events: ElementChangeEvent[]): void {
		for (const event of events) {
			this.renderElement(event);
		}
	}

	private renderComposition(changed: CompositionChange): void {
		// Apply dimensions
		this._width = changed.width;
		this._height = changed.height;
		this._svgElement.setAttribute('width', `${changed.width}px`);
		this._svgElement.setAttribute('height', `${changed.height}px`);

		// Apply view box
		const viewBox = changed.viewBox;
		this._svgElement.setAttribute(
			'viewBox',
			`${viewBox.minX} ${viewBox.minY} ${viewBox.width} ${viewBox.height}`
		);
	}

	private renderElement(renderChange: ElementChangeEvent): void {
		let element: SVGElement | null = null;
		const getElement = (): SVGElement | null => {
			if (element == null) {
				element = this._svgElementMap.get(renderChange.id) ?? null;
			}
			return element;
		};

		for (const change of renderChange.changes) {
			switch (change.type) {
				case 'ElementCreated': {
					const newElement: SVGElement = document.createElementNS(NS, change.tagName);

					// Apply attributes
					for (const attribute of change.attributes) {
						const parsedAttribute = this.parseSVGAttribute(attribute);
						if (parsedAttribute != null) {
							const [key, value] = parsedAttribute;
							newElement.setAttribute(key, value);
						}
					}

					// Apply styles
					for (const style of change.styles) {
						const parsedStyle = this.parseSVGStyle(style);
						if (parsedStyle != null) {
							const [key, value] = parsedStyle;
							newElement.style.setProperty(key, `${value}`);
						}
					}

					// Register callbacks
					const entity = change.entity;
					if (change.isBundleRoot && entity != null) {
						newElement.addEventListener('pointerdown', (e) => {
							e.preventDefault();
							this.composition.emitInteractionEvents([
								{
									type: 'CursorDownOnEntity',
									entity,
									position: this.pointerEventToCompositionPoint(e),
									button: toMouseButton(e.button)
								}
							]);
						});
					}

					this._svgElementMap.set(renderChange.id, newElement);

					// Append element to parent
					if (change.parentId != null) {
						const parentElement = this._svgElementMap.get(change.parentId);
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
						this._svgElementMap.delete(renderChange.id);
					}
					break;
				}
				case 'ElementAppended': {
					const toAppendElement = getElement();
					const parentElement = this._svgElementMap.get(renderChange.id);
					if (parentElement != null && toAppendElement != null) {
						parentElement.appendChild(toAppendElement);
					}
					break;
				}
				case 'AttributeUpdated': {
					const elementToUpdate = getElement();
					if (elementToUpdate != null) {
						const parsedAttribute = this.parseSVGAttribute(change.newValue);
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
						elementToUpdate.removeAttribute(change.key);
					}
					break;
				}
				case 'StyleUpdated': {
					const elementToUpdate = getElement();
					if (elementToUpdate != null) {
						const parsedStyle = this.parseSVGStyle(change.newValue);
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
						elementToUpdate.style.removeProperty(change.key);
					}
					break;
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
			case 'ReferencedFill':
				return ['fill', `url(#${attribute.id})`];
			case 'Name':
				return ['name', attribute.name];
			case 'PatternUnits': {
				switch (attribute.unit.type) {
					case 'UserSpaceOnUse':
						return ['patternUnits', 'userSpaceOnUse'];
					case 'ObjectBoundingBox':
						return ['patternUnits', 'objectBoundingBox'];
					default:
						return ['', ''];
				}
			}
			case 'Href': {
				switch (attribute.href.type) {
					case 'Base64':
						return ['href', `data:image/png;base64,${attribute.href.content}`];
					case 'Url':
						return ['href', attribute.href.url];
					default:
						return ['', ''];
				}
			}
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

	public clientWindowPointToCompositionPoint(clientProint: Vec2): Vec2 {
		const rect = this._svgElement.getBoundingClientRect();

		const x = clientProint[0] - rect.left;
		const y = clientProint[1] - rect.top;

		return [x, y];
	}

	public pointerEventToCompositionPoint(e: PointerEvent): Vec2 {
		return this.clientWindowPointToCompositionPoint([e.clientX, e.clientY]);
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
