import { intoMouseButton } from '@dyn/comp-dtif';
import type { SvgElementChanges, SvgElementId, Vec2 } from '@/rust/dyn-svg-comp-api/bindings';

import type { Composition } from '../Composition';
import { Renderer } from './Renderer';

export const VERSION = '1.1';
export const NS = 'http://www.w3.org/2000/svg';
export const XLINK = 'http://www.w3.org/1999/xlink';

export class SvgRenderer extends Renderer {
	private _domElement: Element;
	private _svgElement: SVGElement;

	private _svgElementMap = new Map<SvgElementId, SVGElement>();

	private _cursorInCompBounds = false;

	constructor(composition: Composition, options: TsvgRendererOptions = {}) {
		super(composition, true);
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
			this.composition.emitInputEvent(
				{
					type: 'Interaction',
					event: {
						type: 'CursorMovedOnComposition',
						position: this.pointerEventToCompositionPoint(e)
					}
				},
				false
			);
		});
		this._svgElement.addEventListener('wheel', (e) => {
			e.preventDefault();
			this.composition.emitInputEvent(
				{
					type: 'Interaction',
					event: {
						type: 'WheeledOnComposition',
						position: this.clientWindowPointToCompositionPoint([e.clientX, e.clientY]),
						ctrlKeyPressed: e.ctrlKey,
						metaKeyPressed: e.metaKey,
						delta: [e.deltaX, e.deltaY]
					}
				},
				false
			);
		});
		this._svgElement.addEventListener('pointerdown', (e) => {
			e.preventDefault();
			this.composition.emitInputEvent(
				{
					type: 'Interaction',
					event: {
						type: 'CursorDownOnComposition',
						position: this.pointerEventToCompositionPoint(e),
						button: intoMouseButton(e.button)
					}
				},
				true
			);
		});
		this._svgElement.addEventListener('pointerup', (e) => {
			e.preventDefault();
			this.composition.emitInputEvent(
				{
					type: 'Interaction',
					event: {
						type: 'CursorUpOnComposition',
						position: this.pointerEventToCompositionPoint(e),
						button: intoMouseButton(e.button)
					}
				},
				true
			);
		});
		this._svgElement.addEventListener('pointerenter', (e) => {
			e.preventDefault();
			if (!this._cursorInCompBounds) {
				this.composition.emitInputEvent(
					{
						type: 'Interaction',
						event: {
							type: 'CursorEnteredComposition'
						}
					},
					false
				);
				this._cursorInCompBounds = true;
			}
		});
		this._svgElement.addEventListener('pointerleave', (e) => {
			e.preventDefault();
			const compositionPoint = this.pointerEventToCompositionPoint(e);
			// Check whether cursor actually left composition
			// or whether its just on some UI layer like the selection box
			if ((this._cursorInCompBounds && compositionPoint[0] < 0) || compositionPoint[1] < 0) {
				this.composition.emitInputEvent(
					{
						type: 'Interaction',
						event: {
							type: 'CursorExitedComposition'
						}
					},
					false
				);
				this._cursorInCompBounds = false;
			}
		});
	}

	public applyElementChanges(elementChanges: SvgElementChanges): void {
		let element: SVGElement | null = null;
		const getElement = (): SVGElement | null => {
			if (element == null) {
				element = this._svgElementMap.get(elementChanges.id) ?? null;
			}
			return element;
		};

		for (const change of elementChanges.changes) {
			switch (change.type) {
				case 'ElementCreated': {
					const newElement: SVGElement = document.createElementNS(NS, change.tagName);

					// Apply attributes
					for (const [key, value] of change.attributes) {
						newElement.setAttribute(key, value);
					}

					// Apply styles
					for (const [key, value] of change.styles) {
						newElement.style.setProperty(key, value);
					}

					// Register callbacks
					const entity = change.entity;
					if (entity != null) {
						newElement.addEventListener('pointerdown', (e) => {
							e.preventDefault();
							this.composition.emitInputEvent({
								type: 'Interaction',
								event: {
									type: 'CursorDownOnEntity',
									entity,
									position: this.pointerEventToCompositionPoint(e),
									button: intoMouseButton(e.button)
								}
							});
						});
					}

					// Append element to parent
					if (change.parentId != null) {
						const parentElement = this._svgElementMap.get(change.parentId);
						if (parentElement != null) {
							parentElement.appendChild(newElement);
						}
					} else {
						this._svgElement.appendChild(newElement);
					}

					this._svgElementMap.set(elementChanges.id, newElement);
					element = newElement;
					break;
				}
				case 'ElementDeleted': {
					const elementToDelete = getElement();
					if (elementToDelete?.parentNode != null) {
						elementToDelete.parentNode.removeChild(elementToDelete);
						this._svgElementMap.delete(elementChanges.id);
					}
					break;
				}
				case 'ElementAppended': {
					const toAppendElement = getElement();
					const parentElement = this._svgElementMap.get(elementChanges.id);
					if (parentElement != null && toAppendElement != null) {
						parentElement.appendChild(toAppendElement);
					}
					break;
				}
				case 'AttributeUpdated': {
					const elementToUpdate = getElement();
					if (elementToUpdate != null) {
						elementToUpdate.setAttribute(change.key, change.newValue);
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
						elementToUpdate.style.setProperty(change.key, change.newValue);
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

	public clear(): void {
		this._svgElementMap.clear();
		while (this._domElement.firstChild) {
			this._domElement.removeChild(this._domElement.firstChild);
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
}

export interface TsvgRendererOptions {
	domElement?: Element;
}
